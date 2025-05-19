use crate::models::user::User;
use crate::services::auth::{generate_tokens, verify_password, verify_token};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login success", body = TokenResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
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

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: access,
        refresh_token: refresh,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = TokenResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
pub async fn refresh_token(
    body: web::Json<RefreshRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let claims = match verify_token(&body.refresh_token, "refresh") {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let (access, refresh) = generate_tokens(&claims.sub);

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: access,
        refresh_token: refresh,
    }))
}
