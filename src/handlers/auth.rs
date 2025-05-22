use crate::services::auth::{authenticate_user, generate_tokens, verify_token};
use crate::validations::validate_login_payload;
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
        (status = 400, description = "Bad request")
    ),
    tag = "auth"
)]
pub async fn login_user(
    pool: web::Data<PgPool>,
    creds: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = validate_login_payload(&creds).await {
        return Ok(err);
    }

    let user = match authenticate_user(&creds.email, &creds.password, pool.get_ref()).await {
        Ok(u) => u,
        Err(resp) => return Ok(resp),
    };

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
