use crate::models::user::User;
use crate::services::auth::hash_password;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,             // "customer" | "provider"
    pub cpf_cnpj: Option<String>, // obrigatório se role = provider
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
        INSERT INTO users (id, name, email, password, role, cpf_cnpj)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, email, password, role, cpf_cnpj
        "#,
        Uuid::new_v4(),
        payload.name,
        payload.email,
        hashed,
        payload.role,
        payload.cpf_cnpj
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
