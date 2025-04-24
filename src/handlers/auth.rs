use crate::models::user::User;
use crate::services::auth::{generate_jwt, verify_password};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password FROM users WHERE email = $1"#,
        credentials.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let user = match user {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    if !verify_password(&user.password, &credentials.password) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let token =
        generate_jwt(&user.id.to_string()).map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}
