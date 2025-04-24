use crate::models::user::User;
use crate::services::auth;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let hash = auth::hash_password(&payload.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash fail"))?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, password
        "#,
        Uuid::new_v4(),
        payload.name,
        payload.email,
        hash
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
