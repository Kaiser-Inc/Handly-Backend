use crate::services::auth::verify_token;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt as _;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use std::{fs, io::Write};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ServiceInfo {
    pub id: Uuid,
    pub category: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub role: String,
    pub profile_pic: Option<String>,
    pub services: Vec<ServiceInfo>,
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
    let key = claims.sub;

    let user = match sqlx::query!(
        "SELECT name, email, role, profile_pic FROM users WHERE cpf_cnpj = $1",
        key
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(u) => u,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let services = match sqlx::query!(
        r#"
        SELECT
          id,
          category,
          name,
          description,
          image,
          to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SSZ') AS created_at,
          to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SSZ') AS updated_at
        FROM services
        WHERE provider_key = $1
        "#,
        key
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|r| ServiceInfo {
                id: r.id,
                category: r.category,
                name: r.name,
                description: r.description,
                image: r.image,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(Profile {
        name: user.name,
        email: user.email,
        role: user.role,
        profile_pic: user.profile_pic,
        services,
    })
}

pub async fn upload_profile_pic(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> HttpResponse {
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
    let key = claims.sub;
    let dir = "./uploads/profile_pics";
    if fs::create_dir_all(dir).is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let mut saved = None;
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) if f.content_disposition().get_name() == Some("file") => f,
            _ => continue,
        };
        let filename = format!("{}.png", Uuid::new_v4());
        let path = format!("{}/{}", dir, &filename);
        let mut f = match fs::File::create(&path) {
            Ok(f) => f,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(bytes) => bytes,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
            if f.write_all(&data).is_err() {
                return HttpResponse::InternalServerError().finish();
            }
        }
        if sqlx::query!(
            "UPDATE users SET profile_pic = $1 WHERE cpf_cnpj = $2",
            filename,
            key
        )
        .execute(pool.get_ref())
        .await
        .is_err()
        {
            return HttpResponse::InternalServerError().finish();
        }
        saved = Some(filename);
        break;
    }
    match saved {
        Some(name) => HttpResponse::Ok().json(json!({ "profile_pic": name })),
        None => HttpResponse::BadRequest().body("file missing"),
    }
}
