use crate::handlers::feed::FeedItem;
use crate::validations::CATEGORIES;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CategoriesQuery {
    categories: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CategoriesResponse {
    pub categories: Vec<&'static str>,
    pub services: Vec<FeedItem>,
}

#[utoipa::path(
    get,
    path = "/categories",
    tag = "Categories",
    params(
        ("categories" = Option<String>, Query,
            description = "Comma-separated category names (max 5)",
            example = "eletricista,encanador")
    ),
    responses(
        (status = 200, description = "OK", body = CategoriesResponse),
        (status = 200, description = "No services", body = Object),
        (status = 400, description = "Invalid filter", body = Object),
        (status = 500, description = "Error", body = Object)
    )
)]
#[get("/categories")]
pub async fn get_categories(
    pool: web::Data<PgPool>,
    query: web::Query<CategoriesQuery>,
) -> HttpResponse {
    let selected: Vec<String> = query
        .categories
        .as_ref()
        .map(|s| {
            s.split(',')
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.trim().to_string())
                .collect()
        })
        .unwrap_or_default();

    if selected.len() > 5 {
        return HttpResponse::BadRequest().json(json!({
            "field": "categories",
            "code": "RN0009",
            "message": "Um campo não foi preenchido corretamente."
        }));
    }

    let rows = if selected.is_empty() {
        sqlx::query(
            "
            SELECT u.name AS provider_name,
                   u.profile_pic,
                   s.id,
                   s.categories,
                   s.name  AS service_name,
                   s.description,
                   s.image
              FROM services s
              JOIN users u ON u.cpf_cnpj = s.provider_key
            ",
        )
        .fetch_all(pool.get_ref())
        .await
    } else {
        sqlx::query(
            "
            SELECT u.name AS provider_name,
                   u.profile_pic,
                   s.id,
                   s.categories,
                   s.name  AS service_name,
                   s.description,
                   s.image
              FROM services s
              JOIN users u ON u.cpf_cnpj = s.provider_key
             WHERE s.categories && $1::text[]
            ",
        )
        .bind(&selected)
        .fetch_all(pool.get_ref())
        .await
    };

    match rows {
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "code": "MA0001",
            "message": "Algo deu errado, tente novamente."
        })),
        Ok(r) if r.is_empty() => HttpResponse::Ok().json(json!({
            "code": "MA0009",
            "message": "Nenhum serviço correspondente."
        })),
        Ok(r) => {
            let services: Vec<FeedItem> = r
                .into_iter()
                .map(|row| FeedItem {
                    provider_name: row.get("provider_name"),
                    profile_pic: row.get("profile_pic"),
                    id: row.get::<Uuid, _>("id").to_string(),
                    categories: row.get("categories"),
                    service_name: row.get("service_name"),
                    description: row.get("description"),
                    image: row.get("image"),
                })
                .collect();

            HttpResponse::Ok().json(CategoriesResponse {
                categories: CATEGORIES.to_vec(),
                services,
            })
        }
    }
}
