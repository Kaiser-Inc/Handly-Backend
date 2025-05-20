use crate::handlers::users::CreateUser;
use crate::validations::ValidationError;
use actix_web::HttpResponse;

/// Validate CreateUser payload according to RN0001–RN0004 business rules.
pub async fn validate_user_payload(
    payload: &CreateUser,
    pool: &sqlx::PgPool,
) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // RN0001: name only letters and spaces
    let name_re = regex::Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]+$").unwrap();
    if !name_re.is_match(&payload.name) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Name must contain only letters and spaces".into(),
        });
    }

    // RN0002: email format and uniqueness
    let email_re = regex::Regex::new(r"^[^@\s]+@[^@\s]+\.(com|br)$").unwrap();
    if !email_re.is_match(&payload.email) {
        errors.push(ValidationError {
            field: "email",
            code: "RN0002",
            message: "Invalid email format".into(),
        });
    } else {
        let exists_opt: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            &payload.email
        )
        .fetch_one(pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
        if exists_opt.unwrap_or(false) {
            errors.push(ValidationError {
                field: "email",
                code: "RN0002",
                message: "Email is already registered".into(),
            });
        }
    }

    // RN0003: password rules
    if payload.password.len() < 8 || payload.password.chars().all(|c| c.is_ascii_digit()) {
        errors.push(ValidationError {
            field: "password",
            code: "RN0003",
            message: "Password must be at least 8 characters and include letters".into(),
        });
    }

    // RN0004: cpf_cnpj required and must be 11 or 14 digits
    if payload
        .cpf_cnpj
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        errors.push(ValidationError {
            field: "cpf_cnpj",
            code: "RN0004",
            message: "CPF/CNPJ is required".into(),
        });
    } else {
        let id = payload.cpf_cnpj.as_ref().unwrap();
        let id_re = regex::Regex::new(r"^\d{11}$|^\d{14}$").unwrap();
        if !id_re.is_match(id) {
            errors.push(ValidationError {
                field: "cpf_cnpj",
                code: "RN0004",
                message: "CPF must be 11 digits or CNPJ 14 digits".into(),
            });
        }
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}
