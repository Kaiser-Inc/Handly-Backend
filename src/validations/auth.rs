use crate::handlers::auth::LoginRequest;
use crate::validations::ValidationError;
use actix_web::HttpResponse;
use regex::Regex;

/// Validate LoginRequest payload according to [RF02] RN0002 & RN0003 business rules,
/// mapping to MA0003 (missing) and MA0006 (invalid credentials).
pub async fn validate_login_payload(payload: &LoginRequest) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // MA0003: missing mandatory fields
    if payload.email.trim().is_empty() {
        errors.push(ValidationError {
            field: "email",
            code: "RN0002",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if payload.password.trim().is_empty() {
        errors.push(ValidationError {
            field: "password",
            code: "RN0003",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    // RN0002: email format → MA0006
    let email_re = Regex::new(r"^[^@\s]+@[^@\s]+\.(com|br)$").unwrap();
    if !email_re.is_match(&payload.email) {
        errors.push(ValidationError {
            field: "email",
            code: "RN0002",
            message: "Credenciais inválidas.".into(),
        });
    }

    // RN0003: password rules → MA0006
    if payload.password.len() < 8 || payload.password.chars().all(|c| c.is_ascii_digit()) {
        errors.push(ValidationError {
            field: "password",
            code: "RN0003",
            message: "Credenciais inválidas.".into(),
        });
    }

    if !errors.is_empty() {
        Err(HttpResponse::BadRequest().json(errors))
    } else {
        Ok(())
    }
}
