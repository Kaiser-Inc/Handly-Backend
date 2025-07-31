use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    handlers::services::{ProviderInfo, ServiceWithProvider},
    models::favorite::FavoriteEntry,
    services::{auth::verify_token, favorites as svc},
};

#[derive(Serialize, ToSchema)]
pub struct PublicProfile {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub role: String,
    #[schema(value_type = Option<String>)]
    pub phone: Option<String>,
    #[schema(value_type = Option<String>)]
    pub profile_pic: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ToggleBody {
    pub target_type: String,
    pub target_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(tag = "target_type", rename_all = "lowercase")]
pub enum FavoriteItem {
    Service {
        #[serde(flatten)]
        service: ServiceWithProvider,
    },
    Provider {
        #[serde(flatten)]
        provider: PublicProfile,
    },
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
    responses((status = 200, body = [FavoriteItem]), (status = 401)),
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

    let basic = svc::list(&pool, &claims.sub)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut out = Vec::with_capacity(basic.len());

    for fav in basic {
        match fav.target_type.as_str() {
            "service" => {
                if let Ok(Some(id)) = Uuid::parse_str(&fav.target_id).map(Some) {
                    if let Ok(svc) = service_with_provider(&pool, id).await {
                        out.push(FavoriteItem::Service { service: svc });
                    }
                }
            }
            "provider" => {
                if let Ok(Some(p)) = provider_profile(&pool, &fav.target_id).await {
                    out.push(FavoriteItem::Provider { provider: p });
                }
            }
            _ => {}
        }
    }

    Ok(HttpResponse::Ok().json(out))
}

async fn provider_profile(pool: &PgPool, key: &str) -> Result<Option<PublicProfile>, sqlx::Error> {
    sqlx::query_as!(
        PublicProfile,
        r#"
        SELECT cpf_cnpj,
               name,
               email,
               role,
               phone       AS "phone?: String",
               profile_pic AS "profile_pic?: String"
          FROM users
         WHERE cpf_cnpj = $1
        "#,
        key
    )
    .fetch_optional(pool)
    .await
}

async fn service_with_provider(
    pool: &PgPool,
    id: Uuid,
) -> Result<ServiceWithProvider, sqlx::Error> {
    let svc = sqlx::query_as!(
        crate::models::service::Service,
        "SELECT id, provider_key, categories, name, description, image, created_at, updated_at \
         FROM services WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

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
