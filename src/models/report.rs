use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

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
