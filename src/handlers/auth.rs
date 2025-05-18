use crate::models::user::User;
use crate::services::auth::{generate_tokens, verify_password, verify_token};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    creds: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT cpf_cnpj, name, email, password, role
        FROM users
        WHERE email = $1
        "#,
        creds.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let user = match user {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    if !verify_password(&user.password, &creds.password) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let (access, refresh) = generate_tokens(&user.cpf_cnpj);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "access_token":  access,
        "refresh_token": refresh
    })))
}

pub async fn refresh_token(
    body: web::Json<RefreshRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let claims = match verify_token(&body.refresh_token, "refresh") {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let (access, refresh) = generate_tokens(&claims.sub);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "access_token":  access,
        "refresh_token": refresh
    })))
}
