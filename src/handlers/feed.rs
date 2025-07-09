use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct FeedItem {
    pub provider_name: String,
    pub id: String,
    pub categories: Vec<String>,
    pub service_name: String,
    pub description: String,
    pub image: Option<String>,
}

#[utoipa::path(
    get,
    path = "/feed",
    tag = "Feed",
    responses(
        (status = 200, description = "Feed retrieved successfully", body = [FeedItem]),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_feed(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        "
        SELECT u.name AS provider_name,
               s.id,
               s.categories,
               s.name AS service_name,
               s.description,
               s.image
          FROM services s
          JOIN users u ON u.cpf_cnpj = s.provider_key
        "
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(records) => {
            let feed: Vec<FeedItem> = records
                .into_iter()
                .map(|r| FeedItem {
                    provider_name: r.provider_name,
                    id: r.id.to_string(),
                    categories: r.categories,
                    service_name: r.service_name,
                    description: r.description,
                    image: r.image,
                })
                .collect();
            HttpResponse::Ok().json(feed)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
