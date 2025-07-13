use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::rating::ServiceRating,
    services::{auth::verify_token, ratings as svc},
    validations::validate_rating_payload,
};

#[derive(Deserialize, ToSchema)]
pub struct RatingBody {
    pub stars: i16,
    pub comment: Option<String>,
}

#[utoipa::path(
    post,
    path = "/services/{id}/ratings",
    security(("bearerAuth" = [])),
    request_body = RatingBody,
    responses(
        (status = 201, description = "MA0010"),
        (status = 400, description = "RN0007"),
        (status = 401),
        (status = 500, description = "MA0001")
    ),
    tag = "services"
)]
pub async fn create_rating(
    req: HttpRequest,
    path: Path<Uuid>,
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

    svc::add_rating(
        &pool,
        path.into_inner(),
        &claims.sub,
        body.stars,
        body.comment.clone(),
    )
    .await
    .map_err(|_| ErrorInternalServerError("MA0001"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "code": "MA0010",
        "message": "Avaliação enviada!"
    })))
}

#[utoipa::path(
    get,
    path = "/services/{id}/ratings",
    responses((status = 200, body = [ServiceRating])),
    tag = "services"
)]
pub async fn list_ratings(
    path: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let items = svc::list_by_service(&pool, path.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(items))
}
