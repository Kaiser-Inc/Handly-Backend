mod auth;
mod service;
mod user;

pub use auth::validate_login_payload;
pub use service::{validate_create_service_payload, validate_update_service_payload};
pub use user::validate_user_payload;

use serde::Serialize;

/// Generic validation error for all payloads.
#[derive(Serialize)]
pub struct ValidationError {
    pub field: &'static str,
    pub code: &'static str,
    pub message: String,
}
