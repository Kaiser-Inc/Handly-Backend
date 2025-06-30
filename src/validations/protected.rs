use actix_web::HttpResponse;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::validations::ValidationError;

static NAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap());

pub fn validate_profile_name(name: &str) -> Result<(), HttpResponse> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(HttpResponse::BadRequest().json([ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        }]));
    }

    if !NAME_RE.is_match(trimmed) {
        return Err(HttpResponse::BadRequest().json([ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        }]));
    }

    Ok(())
}
