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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{web, http::StatusCode};
    use sqlx::PgPool;
    use std::env;

    // ensure those env vars exist so generate_tokens/verify_token won't panic
    fn init_env() {
        env::set_var("JWT_SECRET", "dummy");
        env::set_var("JWT_REFRESH_SECRET", "dummy");
    }

    // lazy pool, no actual connection until used
    fn init_pool() -> PgPool {
        PgPool::connect_lazy("postgres://user:pass@localhost/test_db").unwrap()
    }

    #[actix_web::test]
    async fn login_missing_fields_returns_400() {
        init_env();
        let pool = web::Data::new(init_pool());
        let creds = web::Json(LoginRequest {
            email: "".into(),
            password: "".into(),
        });
        let resp = login_user(pool, creds).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn login_invalid_credentials_returns_401() {
        init_env();
        let pool = web::Data::new(init_pool());
        let creds = web::Json(LoginRequest {
            email: "no@user.com".into(),
            password: "wrong".into(),
        });
        let resp = login_user(pool, creds).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn refresh_invalid_token_returns_401() {
        init_env();
        let body = web::Json(RefreshRequest {
            refresh_token: "bad.token.here".into(),
        });
        let resp = refresh_token(body).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn refresh_valid_token_returns_200() {
        init_env();
        // generate a valid refresh
        let (_access, refresh) = generate_tokens("test-user");
        let body = web::Json(RefreshRequest { refresh_token: refresh });
        let resp = refresh_token(body).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
