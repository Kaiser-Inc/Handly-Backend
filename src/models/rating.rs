use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{serde::rfc3339, OffsetDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ServiceRating {
    pub id: Uuid,
    pub service_id: Uuid,
    pub user_id: String,
    pub stars: i16,
    pub comment: Option<String>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}
