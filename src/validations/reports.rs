use crate::handlers::reports::ReportPayload;
use crate::validations::ValidationError;
use actix_web::HttpResponse;

pub fn validate_report_payload(p: &ReportPayload) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    if p.reason.is_none() {
        errors.push(ValidationError {
            field: "reason",
            code: "RN0008",
            message: "Selecione um motivo.".into(),
        });
    }

    if let Some(desc) = p.description.as_ref() {
        let len = desc.trim().chars().count();
        if len < 10 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0009",
                message: "A descrição deve ter no mínimo 10 caracteres.".into(),
            });
        } else if len > 300 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0009",
                message: "A descrição deve ter no máximo 300 caracteres.".into(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest().json(errors))
    }
}
