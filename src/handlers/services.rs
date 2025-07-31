use crate::validations::{validate_create_service_payload, validate_update_service_payload};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::future::try_join_all;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::service::Service;
use crate::services::auth::verify_token;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ProviderInfo {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub role: String,
    #[schema(value_type = Option<String>)]
    pub phone: Option<String>,
    #[schema(value_type = Option<String>)]
    pub profile_pic: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ServiceWithProvider {
    pub id: Uuid,
    pub provider_key: String,
    pub categories: Vec<String>,
    pub name: String,
    pub description: String,
    #[schema(value_type = Option<String>)]
    pub image: Option<String>,

    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: OffsetDateTime,

    pub provider: ProviderInfo,
}

async fn service_with_provider(
    pool: &PgPool,
    svc: Service,
) -> Result<ServiceWithProvider, sqlx::Error> {
    let u = sqlx::query!(
        r#"
        SELECT cpf_cnpj,
               name,
               email,
               role,
               phone        AS "phone?: String",
               profile_pic  AS "profile_pic?: String"
          FROM users
         WHERE cpf_cnpj = $1
        "#,
        svc.provider_key
    )
    .fetch_one(pool)
    .await?;

    Ok(ServiceWithProvider {
        id: svc.id,
        provider_key: svc.provider_key.clone(),
        categories: svc.categories,
        name: svc.name,
        description: svc.description,
        image: svc.image,
        created_at: svc.created_at,
        updated_at: svc.updated_at,
        provider: ProviderInfo {
            cpf_cnpj: u.cpf_cnpj,
            name: u.name,
            email: u.email,
            role: u.role,
            phone: u.phone,
            profile_pic: u.profile_pic,
        },
    })
}

#[derive(Deserialize, ToSchema)]
pub struct CreateService {
    pub categories: Vec<String>,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateService {
    pub categories: Vec<String>,
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
        (status = 201, description = "Service created", body = ServiceWithProvider),
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
    let claims = verify_token(token, "access").ok_or_else(|| {
        ErrorUnauthorized(json!({
            "code": "MA0006",
            "message": "Credenciais inválidas."
        }))
    })?;
    let provider_key = claims.sub;

    let svc: Service = sqlx::query_as!(
        Service,
        r#"
        INSERT INTO services
              (id, provider_key, categories, name, description, image)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, provider_key, categories, name, description, image, created_at, updated_at
        "#,
        Uuid::new_v4(),
        provider_key,
        &payload.categories,
        payload.name,
        payload.description,
        payload.image
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| {
        ErrorInternalServerError(json!({
            "code": "MA0001",
            "message": "Algo deu errado, tente novamente."
        }))
    })?;

    let svc_full = service_with_provider(pool.get_ref(), svc)
        .await
        .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Created().json(json!({
        "code": "MA0005",
        "message": "Cadastro feito com sucesso.",
        "service": svc_full
    })))
}

#[utoipa::path(
    put,
    path = "/services/{id}",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    request_body = UpdateService,
    responses(
        (status = 200, description = "Service updated", body = ServiceWithProvider),
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
           SET categories = $2,
               name = $3,
               description = $4,
               image = $5,
               updated_at = NOW()
         WHERE id = $1
        RETURNING id, provider_key, categories, name, description, image, created_at, updated_at
        "#,
        id,
        &payload.categories,
        payload.name,
        payload.description,
        payload.image
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| {
        ErrorInternalServerError(json!({
            "code": "MA0001",
            "message": "Algo deu errado, tente novamente."
        }))
    })?;

    let svc_full = service_with_provider(pool.get_ref(), svc)
        .await
        .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(json!({
        "code": "MA0007",
        "message": "Alterações feitas com sucesso.",
        "service": svc_full
    })))
}

#[utoipa::path(
    get,
    path = "/services",
    responses(
        (status = 200, description = "List services", body = [ServiceWithProvider]),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn list_services(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let services_raw: Vec<Service> = sqlx::query_as!(
        Service,
        "SELECT id, provider_key, categories, name, description, image, created_at, updated_at FROM services ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("DB error"))?;

    let futs = services_raw
        .into_iter()
        .map(|s| service_with_provider(pool.get_ref(), s));
    let services: Vec<ServiceWithProvider> = try_join_all(futs)
        .await
        .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(services))
}

#[utoipa::path(
    get,
    path = "/services/{id}",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    responses(
        (status = 200, description = "Get service", body = ServiceWithProvider),
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
        "SELECT id, provider_key, categories, name, description, image, created_at, updated_at FROM services WHERE id=$1",
        id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| ErrorInternalServerError("Not found"))?;

    let svc_full = service_with_provider(pool.get_ref(), svc)
        .await
        .map_err(|_| ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(svc_full))
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

#[utoipa::path(
    get,
    path = "/services/{id}/image",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    responses(
        (status = 200, description = "Image file returned"),
        (status = 404, description = "Imagem não encontrada"),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn get_service_image(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let id = path.into_inner();
    let filename: Option<String> =
        match sqlx::query_scalar!("SELECT image FROM services WHERE id = $1", id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(v) => v,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
    let name = match filename {
        Some(n) => n,
        None => return HttpResponse::NoContent().finish(),
    };
    let full_path: PathBuf = ["./uploads/services", &name].iter().collect();
    if full_path.exists() {
        if let Ok(file) = NamedFile::open(full_path) {
            return file.into_response(&req);
        }
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::NoContent().finish()
}

#[utoipa::path(
    get,
    path = "/services/{id}/publisher/profilepic",
    params(("id" = String, Path, description = "Service ID", example = "550e8400-e29b-41d4-a716-446655440000")),
    responses(
        (status = 200, description = "Profile picture returned"),
        (status = 404, description = "Profile picture not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "services"
)]
pub async fn get_publisher_profile_pic(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let id = path.into_inner();
    let provider_key: String =
        match sqlx::query_scalar!("SELECT provider_key FROM services WHERE id = $1", id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(v) => v,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
    let filename: Option<String> = match sqlx::query_scalar!(
        "SELECT profile_pic FROM users WHERE cpf_cnpj = $1",
        provider_key
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let name = match filename {
        Some(n) => n,
        None => return HttpResponse::NoContent().finish(),
    };
    let full_path: PathBuf = ["./uploads/profile_pics", &name].iter().collect();
    if full_path.exists() {
        if let Ok(file) = NamedFile::open(full_path) {
            return file.into_response(&req);
        }
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::NoContent().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    fn init_pool() -> PgPool {
        // lazy pool, no actual connection until used
        PgPool::connect_lazy("postgres://user:pass@localhost/db").unwrap()
    }

    #[actix_web::test]
    async fn create_service_unauthorized() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(init_pool()))
                .route("/services", web::post().to(create_service)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/services")
            .set_json(&json!({
                "categories": ["eletricista"],
                "name": "Test",
                "description": "Desc",
                "image": null
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn update_service_unauthorized() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(init_pool()))
                .route("/services/{id}", web::put().to(update_service)),
        )
        .await;

        let req = test::TestRequest::put()
            .uri(&format!("/services/{}", Uuid::nil()))
            .set_json(&json!({
                "categories": ["eletricista"],
                "name": "Test",
                "description": "Desc",
                "image": null
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn upload_service_image_unauthorized() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(init_pool()))
                .route("/services/{id}/image", web::post().to(upload_service_image)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&format!("/services/{}/image", Uuid::nil()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
