use crate::handlers::reports::ReportPayload;
use crate::validations::ValidationError;
use actix_web::HttpResponse;

pub fn validate_report_payload(p: &ReportPayload) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // RN0008 – motivo obrigatório
    if p.reason.is_none() {
        errors.push(ValidationError {
            field: "reason",
            code: "RN0008",
            message: "Selecione um motivo.".into(), // MA0003
        });
    }

    // RN0009 - description optional, but if provided, must be <= 250 characters
    if let Some(desc) = &p.description {
        if desc.len() < 10 || desc.len() > 300 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0009",
                message: "Máximo 300 caracteres.".into(), // MA0004 + MA0034
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest().json(errors))
    }
}
