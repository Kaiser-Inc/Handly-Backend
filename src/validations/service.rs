use crate::handlers::services::{CreateService, UpdateService};
use crate::validations::ValidationError;
use actix_web::HttpResponse;

const CATEGORIES: &[&str] = &[
    "eletricista",
    "encanador",
    "pedreiro",
    "pintor",
    "montador de móveis",
    "técnico em ar-condicionado",
    "diarista",
    "mototaxista",
    "motorista particular",
    "entregador",
    "freteiro",
    "guincheiro",
    "doceria",
    "marmitaria",
    "buffet",
    "confeitaria personalizada",
    "padaria artesanal",
    "cabeleireira",
    "manicure",
    "maquiadora",
    "designer de sobrancelhas",
    "esteticista",
    "professor particular",
    "professor de música",
    "professor de idiomas",
    "personal trainer",
    "técnico de informática",
    "técnico de celular",
    "instalador de câmeras",
    "pet shop",
    "chaveiro",
    "costureira",
    "babá",
    "cuidador de idosos",
    "lava a jato",
    "vidraceiro",
    "marceneiro",
    "fotógrafo",
    "film maker",
    "segurança",
    "garçom",
    "massoterapia",
    "psicólogo",
    "designer gráfico",
    "social media",
    "mecânico",
];

pub async fn validate_create_service_payload(payload: &CreateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // RN0006: description must be at most 300 characters
    if payload.description.chars().count() > 300 {
        errors.push(ValidationError {
            field: "description",
            code: "RN0006",
            message: "Description must be at most 300 characters".into(),
        });
    }

    // RN0007: category must be one of the allowed options
    if !CATEGORIES.contains(&payload.category.as_str()) {
        errors.push(ValidationError {
            field: "category",
            code: "RN0007",
            message: format!("Category must be one of: {}", CATEGORIES.join(", ")),
        });
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}

pub async fn validate_update_service_payload(payload: &UpdateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // RN0006: description must be at most 300 characters
    if payload.description.chars().count() > 300 {
        errors.push(ValidationError {
            field: "description",
            code: "RN0006",
            message: "Description must be at most 300 characters".into(),
        });
    }

    // RN0007: category must be one of the allowed options
    if !CATEGORIES.contains(&payload.category.as_str()) {
        errors.push(ValidationError {
            field: "category",
            code: "RN0007",
            message: format!("Category must be one of: {}", CATEGORIES.join(", ")),
        });
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}
