use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use sqlx::PgPool;

use crate::{
    handlers::ratings::RatingBody,
    models::provider_rating::ProviderRating,
    services::{auth::verify_token, provider_rating as svc},
    validations::validate_rating_payload,
};

#[utoipa::path(
    post,
    path = "/providers/{id}/ratings",
    security(("bearerAuth" = [])),
    request_body = RatingBody,
    responses(
        (status = 201, description = "MA0010"),
        (status = 400, description = "RN0007"),
        (status = 401),
        (status = 500, description = "MA0001")
    ),
    tag = "users"
)]
pub async fn create_rating(
    req: HttpRequest,
    path: Path<String>,
    pool: Data<PgPool>,
    body: Json<RatingBody>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(resp) = validate_rating_payload(&body) {
        return Ok(resp);
    }

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

    svc::add_rating(&pool, &path, &claims.sub, body.stars, body.comment.clone())
        .await
        .map_err(|_| ErrorInternalServerError("MA0001"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "code": "MA0010",
        "message": "Avaliação enviada!"
    })))
}

#[utoipa::path(
    get,
    path = "/providers/{id}/ratings",
    responses((status = 200, body = [ProviderRating])),
    tag = "users"
)]
pub async fn list_ratings(
    path: Path<String>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let items = svc::list_by_provider(&pool, &path)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(items))
}
