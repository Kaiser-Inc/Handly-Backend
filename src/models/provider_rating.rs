use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{serde::rfc3339, OffsetDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProviderRating {
    pub id: Uuid,
    pub provider_id: String,
    pub user_id: String,

    #[schema(example = 4, minimum = 1, maximum = 5)]
    pub stars: i16,

    #[schema(example = "Muito bom!")]
    pub comment: Option<String>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}
