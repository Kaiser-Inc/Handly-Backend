use actix_web::HttpResponse;
use crate::handlers::users::CreateUser;
use crate::validations::ValidationError;
use regex::Regex;
use sqlx::PgPool;

/// Validate CreateUser payload according to RN0001–RN0004 business rules,
/// mapping to MA0002–MA0004 messages.
pub async fn validate_user_payload(
    payload: &CreateUser,
    pool:    &PgPool,
) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // missing mandatory fields → MA0003
    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field:   "name",
            code:    "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.email.trim().is_empty() {
        errors.push(ValidationError {
            field:   "email",
            code:    "RN0002",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.password.trim().is_empty() {
        errors.push(ValidationError {
            field:   "password",
            code:    "RN0003",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.cpf_cnpj.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        errors.push(ValidationError {
            field:   "cpf_cnpj",
            code:    "RN0004",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }

    // abort early if any missing
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    // RN0001: name content → MA0004
    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]+$").unwrap();
    if !name_re.is_match(&payload.name) {
        errors.push(ValidationError {
            field:   "name",
            code:    "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0002: email format → MA0004
    let email_re = Regex::new(r"^[^@\s]+@[^@\s]+\.(com|br)$").unwrap();
    if !email_re.is_match(&payload.email) {
        errors.push(ValidationError {
            field:   "email",
            code:    "RN0002",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    } else {
        // RN0002: email uniqueness → MA0002
        let exists_opt: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            &payload.email
        )
        .fetch_one(pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
        if exists_opt.unwrap_or(false) {
            errors.push(ValidationError {
                field:   "email",
                code:    "RN0002",
                message: "E-mail já está cadastrado no sistema.".into(), // MA0002
            });
        }
    }

    // RN0003: password rules → MA0004
    if payload.password.len() < 8 || payload.password.chars().all(|c| c.is_ascii_digit()) {
        errors.push(ValidationError {
            field:   "password",
            code:    "RN0003",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0004: CPF/CNPJ format → MA0004
    let id = payload.cpf_cnpj.as_ref().unwrap();
    let id_re = Regex::new(r"^\d{11}$|^\d{14}$").unwrap();
    if !id_re.is_match(id) {
        errors.push(ValidationError {
            field:   "cpf_cnpj",
            code:    "RN0004",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }
    
    // RN0004: CPF/CNPJ must be exactly 11 or 14 digits → MA0004
    let id = payload.cpf_cnpj.as_ref().unwrap();
    let id_re = Regex::new(r"^\d{11}$|^\d{14}$").unwrap();
    if !id_re.is_match(id) {
        errors.push(ValidationError {
            field:   "cpf_cnpj",
            code:    "RN0004",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    if !errors.is_empty() {
        Err(HttpResponse::BadRequest().json(errors))
    } else {
        Ok(())
    }
}
