use crate::services::auth::verify_token;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub role: String,
    pub profile_pic: Option<String>,
    // TODO: services: Vec<ServiceInfo>,
}

pub async fn get_profile(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    let claims = match verify_token(token, "access") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let cpf = &claims.sub;

    let row = match sqlx::query!(
        r#"
        SELECT name, email, role, profile_pic
          FROM users
         WHERE cpf_cnpj = $1
        "#,
        cpf
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(Profile {
        name: row.name,
        email: row.email,
        role: row.role,
        profile_pic: row.profile_pic,
    })
}
