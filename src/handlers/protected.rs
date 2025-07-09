use crate::models::service::Service;
use crate::services::auth::verify_token;
use crate::validations::validate_profile_name;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::path::PathBuf;
use std::{fs, io::Write};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub role: String,
    pub profile_pic: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ProfilePicResponse {
    pub profile_pic: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateProfile {
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/protected/profile",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Profile info", body = Profile),
        (status = 401, description = "Unauthorized")
    ),
    tag = "protected"
)]
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
    HttpResponse::Ok().json(Profile {
        name: user.name,
        email: user.email,
        role: user.role,
        profile_pic: user.profile_pic,
    })
}

#[utoipa::path(
    put,
    path = "/protected/profile",
    security(("bearerAuth" = [])),
    request_body = UpdateProfile,
    responses(
        (status = 200, description = "Profile updated", body = Profile),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "protected"
)]
pub async fn update_profile(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    payload: web::Json<UpdateProfile>,
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

    if let Err(resp) = validate_profile_name(&payload.name) {
        return resp;
    }

    let row = match sqlx::query!(
        "UPDATE users SET name = $1 WHERE cpf_cnpj = $2 \
         RETURNING name, email, role, profile_pic",
        payload.name.trim(),
        key
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "code": "MA0001",
                "message": "Algo deu errado, tente novamente."
            }));
        }
    };

    HttpResponse::Ok().json(Profile {
        name: row.name,
        email: row.email,
        role: row.role,
        profile_pic: row.profile_pic,
    })
}

#[utoipa::path(
    post,
    path = "/protected/profilepic",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Upload success", body = ProfilePicResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "protected"
)]
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
        Some(name) => HttpResponse::Ok().json(ProfilePicResponse { profile_pic: name }),
        None => HttpResponse::BadRequest().body("file missing"),
    }
}

#[utoipa::path(
    get,
    path = "/protected/services",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "List services", body = [Service]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "protected"
)]
pub async fn get_user_services(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
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
    let provider_key = claims.sub;
    let services = match sqlx::query_as!(
        Service,
        "SELECT id, provider_key, categories, name, description, image, created_at, updated_at \
         FROM services \
         WHERE provider_key = $1",
        provider_key
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(list) => list,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().json(services)
}

#[utoipa::path(
    get,
    path = "/protected/profilepic",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Profile picture returned"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Profile picture not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "protected"
)]
pub async fn get_profile_pic(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> actix_web::Result<NamedFile> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");

    let claims = match verify_token(token, "access") {
        Some(c) => c,
        None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    };

    let key = claims.sub;
    let user = sqlx::query!("SELECT profile_pic FROM users WHERE cpf_cnpj = $1", key)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB Error"))?;

    let filename = match user.profile_pic {
        Some(name) => name,
        None => return Err(actix_web::error::ErrorNotFound("No profile picture set")),
    };

    let path: PathBuf = format!("./uploads/profile_pics/{filename}").into();

    if !path.exists() {
        return Err(actix_web::error::ErrorNotFound("File not found"));
    }

    Ok(NamedFile::open(path)?)
}
