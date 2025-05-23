use crate::services::auth::hash_password;
use crate::validations::validate_user_payload;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub cpf_cnpj: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = MessageResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = validate_user_payload(&payload, pool.get_ref()).await {
        return Ok(err);
    }
    let hashed = hash_password(&payload.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash_fail"))?;
    sqlx::query!(
        "INSERT INTO users (cpf_cnpj, name, email, password, role) VALUES ($1, $2, $3, $4, $5)",
        payload.cpf_cnpj.as_deref(),
        &payload.name,
        &payload.email,
        &hashed,
        &payload.role,
    )
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(MessageResponse {
        message: "Cadastro feito com sucesso.".into(),
    }))
}
