use crate::services::auth::verify_token;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use std::{fs, io::Write};
use uuid::Uuid;

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
    let key = claims.sub;

    let row = match sqlx::query!(
        "SELECT name, email, role, profile_pic FROM users WHERE cpf_cnpj = $1",
        key
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

    let dir = "./uploads/avatars";
    if fs::create_dir_all(dir).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let mut saved: Option<String> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() != "file" {
            continue;
        }

        let filename = format!("{}.png", Uuid::new_v4());
        let path = format!("{}/{}", dir, &filename);
        let mut f = match fs::File::create(&path) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        while let Ok(Some(bytes)) = field.try_next().await {
            if f.write_all(&bytes).is_err() {
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
