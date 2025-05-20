use crate::models::user::User;
use crate::services::auth::hash_password;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use regex::Regex;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub cpf_cnpj: Option<String>,
}

#[derive(Serialize)]
struct ValidationError {
    field: &'static str,
    code:   &'static str,
    message: String,
}

async fn validate_create_user(
    payload: &CreateUser,
    pool:    &PgPool,
) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // RN0001: name only letters and spaces
    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]+$").unwrap();
    if !name_re.is_match(&payload.name) {
        errors.push(ValidationError {
            field: "name",
            code:  "RN0001",
            message: "Name must contain only letters and spaces".into(),
        });
    }

    // RN0002: email format
    let email_re = Regex::new(r"^[^@\s]+@[^@\s]+\.(com|br)$").unwrap();
    if !email_re.is_match(&payload.email) {
        errors.push(ValidationError {
            field: "email",
            code:  "RN0002",
            message: "Invalid email format".into(),
        });
    } else {
        // RN0002: email uniqueness
        let exists_opt: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            &payload.email
        )
        .fetch_one(pool)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;
        if exists_opt.unwrap_or(false) {
            errors.push(ValidationError {
                field: "email",
                code:  "RN0002",
                message: "Email is already registered".into(),
            });
        }
    }

    // RN0003: password rules
    if payload.password.len() < 8
        || payload.password.chars().all(|c| c.is_ascii_digit())
    {
        errors.push(ValidationError {
            field: "password",
            code:  "RN0003",
            message: "Password must be at least 8 chars and include letters".into(),
        });
    }

    // RN0004: cpf/cnpj required and must be 11 or 14 digits
    if payload.cpf_cnpj.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        errors.push(ValidationError {
            field: "cpf_cnpj",
            code:  "RN0004",
            message: "CPF/CNPJ is required".into(),
        });
    } else {
        let id = payload.cpf_cnpj.as_ref().unwrap();
        let id_re = Regex::new(r"^\d{11}$|^\d{14}$").unwrap();
        if !id_re.is_match(id) {
            errors.push(ValidationError {
                field: "cpf_cnpj",
                code:  "RN0004",
                message: "CPF must be 11 digits or CNPJ 14 digits".into(),
            });
        }
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = validate_create_user(&payload, pool.get_ref()).await {
        return Ok(err);
    }

    let hashed = hash_password(&payload.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash_fail"))?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (cpf_cnpj, name, email, password, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING cpf_cnpj, name, email, password, role
        "#,
        payload.cpf_cnpj.as_deref(),
        &payload.name,
        &payload.email,
        &hashed,
        &payload.role,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
