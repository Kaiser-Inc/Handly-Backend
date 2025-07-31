mod auth;
mod protected;
mod ratings;
mod reports;
mod services;
mod user;

pub use auth::validate_login_payload;
pub use protected::validate_profile_name;
pub use protected::validate_profile_phone;
pub use ratings::validate_rating_payload;
pub use reports::validate_report_payload;
pub use services::{validate_create_service_payload, validate_update_service_payload, CATEGORIES};
pub use user::validate_user_payload;

use serde::Serialize;

#[derive(Serialize)]
pub struct ValidationError {
    pub field: &'static str,
    pub code: &'static str,
    pub message: String,
}

#[derive(Serialize)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}
