use crate::validations::{validate_create_service_payload, validate_update_service_payload};
use actix_multipart::Multipart;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fs;
use std::io::Write;
use uuid::Uuid;

use crate::models::service::Service;
use crate::services::auth::verify_token;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateService {
    pub category: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateService {
    pub category: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ImageResponse {
    pub image: String,
}

#[utoipa::path(
    post,
    path = "/services",
    request_body = CreateService,
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Service created", body = Service),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn create_service(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    payload: web::Json<CreateService>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = validate_create_service_payload(&payload).await {
        return Ok(err);
    }

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    let claims = verify_token(token, "access").ok_or_else(|| ErrorUnauthorized("Invalid token"))?;
    let provider_key = claims.sub;

    let svc: Service = sqlx::query_as!(
        Service,
        r#"
        INSERT INTO services
          (id, provider_key, category, name, description, image)
        VALUES
          ($1, $2, $3, $4, $5, $6)
        RETURNING
          id, provider_key, category, name, description, image, created_at, updated_at
        "#,
        Uuid::new_v4(),
        provider_key,
        payload.category,
        payload.name,
        payload.description,
        payload.image
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Created().json(svc))
}

#[utoipa::path(
    put,
    path = "/services/{id}",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    request_body = UpdateService,
    responses(
        (status = 200, description = "Service updated", body = Service),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn update_service(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateService>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = validate_update_service_payload(&payload).await {
        return Ok(err);
    }

    let id = path.into_inner();
    let svc: Service = sqlx::query_as!(
        Service,
        r#"
        UPDATE services
           SET category = $2,
               name = $3,
               description = $4,
               image = $5,
               updated_at = NOW()
         WHERE id = $1
        RETURNING id, provider_key, category, name, description, image, created_at, updated_at
        "#,
        id,
        payload.category,
        payload.name,
        payload.description,
        payload.image
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(svc))
}

#[utoipa::path(
    get,
    path = "/services",
    responses(
        (status = 200, description = "List services", body = [Service]),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn list_services(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let services: Vec<Service> = sqlx::query_as!(
        Service,
        "SELECT id, provider_key, category, name, description, image, created_at, updated_at FROM services"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(services))
}

#[utoipa::path(
    get,
    path = "/services/{id}",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    responses(
        (status = 200, description = "Get service", body = Service),
        (status = 500, description = "Not found")
    ),
    tag = "services"
)]
pub async fn get_service(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let svc: Service = sqlx::query_as!(
        Service,
        "SELECT id, provider_key, category, name, description, image, created_at, updated_at FROM services WHERE id=$1", id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("Not found"))?;

    Ok(HttpResponse::Ok().json(svc))
}

#[utoipa::path(
    delete,
    path = "/services/{id}",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    responses(
        (status = 204, description = "Service deleted"),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn delete_service(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    sqlx::query!("DELETE FROM services WHERE id=$1", id)
        .execute(pool.get_ref())
        .await
        .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    post,
    path = "/services/{id}/image",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Image uploaded", body = ImageResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn upload_service_image(
    path: web::Path<Uuid>,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> HttpResponse {
    let service_id = path.into_inner();
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
    let owner =
        match sqlx::query_scalar!("SELECT provider_key FROM services WHERE id=$1", service_id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(o) => o,
            Err(_) => return HttpResponse::NotFound().finish(),
        };
    if owner != provider_key {
        return HttpResponse::Forbidden().finish();
    }
    let dir = "./uploads/services";
    if fs::create_dir_all(dir).is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) if f.content_disposition().get_name() == Some("file") => f,
            _ => continue,
        };
        let filename = format!("{}.png", Uuid::new_v4());
        let filepath = format!("{}/{}", dir, &filename);
        let mut f = match fs::File::create(&filepath) {
            Ok(f) => f,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
        while let Some(chunk) = field.next().await {
            let data: Bytes = match chunk {
                Ok(bytes) => bytes,
                Err(_) => Bytes::new(),
            };
            if f.write_all(&data).is_err() {
                return HttpResponse::InternalServerError().finish();
            }
        }
        if sqlx::query!(
            "UPDATE services SET image=$1 WHERE id=$2",
            filename,
            service_id
        )
        .execute(pool.get_ref())
        .await
        .is_err()
        {
            return HttpResponse::InternalServerError().finish();
        }
        return HttpResponse::Ok().json(ImageResponse { image: filename });
    }
    HttpResponse::BadRequest().body("file missing")
}
