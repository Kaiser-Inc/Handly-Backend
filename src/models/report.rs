use serde::{Deserialize, Serialize};
use sqlx::Type;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportReason {
    InnapropiateContent,
    Fraud,
    Harassment,
    Spam,
    Other,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct Report {
    pub id: Uuid,
    pub reporter_key: String,
    pub target_type: String,
    pub target_id: String,
    pub reason_code: ReportReason,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
}
