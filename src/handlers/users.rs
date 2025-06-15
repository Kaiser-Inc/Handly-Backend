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

#[cfg(test)]
mod tests {
    use crate::handlers::users::CreateUser;
    use crate::validations::validate_user_payload;
    use actix_web::{http::StatusCode, web};
    use sqlx::PgPool;

    // lazy pool, no actual connection until used
    fn init_pool() -> PgPool {
        PgPool::connect_lazy("postgres://user:pass@localhost/fake_db").unwrap()
    }

    #[actix_web::test]
    async fn rejects_missing_mandatory_fields() {
        let pool = init_pool();
        let payload = CreateUser {
            name: "".into(),
            email: "".into(),
            password: "".into(),
            role: "".into(),
            cpf_cnpj: None,
        };
        let result = validate_user_payload(&web::Json(payload), &pool).await;
        assert!(result.is_err());
        let resp = result.unwrap_err();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn rejects_invalid_name_format() {
        let pool = init_pool();
        let payload = CreateUser {
            name: "X".into(), // too short or invalid chars
            email: "user@example.com".into(),
            password: "Password1!".into(),
            role: "user".into(),
            cpf_cnpj: Some("12345678901".into()),
        };
        let result = validate_user_payload(&web::Json(payload), &pool).await;
        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn rejects_invalid_email_format() {
        let pool = init_pool();
        let payload = CreateUser {
            name: "Valid Name".into(),
            email: "invalid-email".into(), // missing @domain
            password: "Password1!".into(),
            role: "user".into(),
            cpf_cnpj: Some("12345678901".into()),
        };
        let result = validate_user_payload(&web::Json(payload), &pool).await;
        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn rejects_short_password() {
        let pool = init_pool();
        let payload = CreateUser {
            name: "Valid Name".into(),
            email: "user@example.com".into(),
            password: "short".into(), // too short
            role: "user".into(),
            cpf_cnpj: Some("12345678901".into()),
        };
        let result = validate_user_payload(&web::Json(payload), &pool).await;
        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn rejects_invalid_cpf_cnpj() {
        let pool = init_pool();
        let payload = CreateUser {
            name: "Valid Name".into(),
            email: "user@example.com".into(),
            password: "Password1!".into(),
            role: "user".into(),
            cpf_cnpj: Some("00000000000".into()), // invalid repetition
        };
        let result = validate_user_payload(&web::Json(payload), &pool).await;
        assert!(result.is_err());
    }
}
