use crate::{
    handlers::ratings::RatingBody,
    validations::{ValidationError, ValidationErrors},
};
use actix_web::HttpResponse;

pub fn validate_rating_payload(body: &RatingBody) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    if !(1..=5).contains(&body.stars) {
        errors.push(ValidationError {
            field: "stars",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }

    if let Some(c) = &body.comment {
        let len = c.trim().chars().count();
        if !(10..=200).contains(&len) {
            errors.push(ValidationError {
                field: "comment",
                code: "RN0007",
                message: "Um campo não foi preenchido corretamente.".into(), // MA0004
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest().json(ValidationErrors { errors }))
    }
}
