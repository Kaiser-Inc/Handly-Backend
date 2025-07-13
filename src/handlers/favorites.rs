use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    models::favorite::FavoriteEntry,
    services::{auth::verify_token, favorites as svc},
};

#[derive(Deserialize, ToSchema)]
pub struct ToggleBody {
    pub target_type: String,
    pub target_id: String,
}

#[utoipa::path(
    post,
    path = "/favorites",
    request_body = ToggleBody,
    responses((status = 201), (status = 204), (status = 401)),
    tag = "favorites"
)]
pub async fn toggle_favorite(
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<ToggleBody>,
) -> Result<HttpResponse, actix_web::Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    let claims = match verify_token(token, "access") {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let added = svc::toggle(
        &pool,
        &claims.sub,
        FavoriteEntry {
            target_type: body.target_type.clone(),
            target_id: body.target_id.clone(),
        },
    )
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(if added {
        HttpResponse::Created().finish()
    } else {
        HttpResponse::NoContent().finish()
    })
}

#[utoipa::path(
    get,
    path = "/favorites",
    responses((status = 200, body = [FavoriteEntry]), (status = 401)),
    tag = "favorites"
)]
pub async fn list_favorites(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    let claims = match verify_token(token, "access") {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    let favs = svc::list(&pool, &claims.sub)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(favs))
}
