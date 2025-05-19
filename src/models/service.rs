use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::rfc3339;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Service {
    #[schema(value_type = String, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub provider_key: String,
    pub category: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    #[schema(value_type = String, example = "2025-05-19T12:34:56Z")]
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[schema(value_type = String, example = "2025-05-19T12:34:56Z")]
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
}
