use crate::models::user::User;
use crate::services::auth::hash_password;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,             // "customer" | "provider"
    pub cpf_cnpj: Option<String>, // obligatory
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> Result<HttpResponse, actix_web::Error> {
    if payload.role == "provider" && payload.cpf_cnpj.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let hashed = hash_password(&payload.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash fail"))?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (cpf_cnpj, name, email, password, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING cpf_cnpj, name, email, password, role
        "#,
        payload
            .cpf_cnpj
            .as_ref()
            .ok_or_else(|| actix_web::error::ErrorBadRequest("cpf_cnpj required"))?,
        payload.name,
        payload.email,
        hashed,
        payload.role,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
