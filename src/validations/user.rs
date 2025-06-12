use crate::handlers::users::CreateUser;
use crate::validations::ValidationError;
use actix_web::HttpResponse;
use regex::Regex;
use sqlx::PgPool;

/// Validate CreateUser payload according to [RF01] RN0001–RN0004 business rules,
/// mapping to MA0002–MA0004 messages.
pub async fn validate_user_payload(
    payload: &CreateUser,
    pool: &PgPool,
) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // missing mandatory fields → MA0003
    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.email.trim().is_empty() {
        errors.push(ValidationError {
            field: "email",
            code: "RN0002",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.password.trim().is_empty() {
        errors.push(ValidationError {
            field: "password",
            code: "RN0003",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload
        .cpf_cnpj
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        errors.push(ValidationError {
            field: "cpf_cnpj",
            code: "RN0004",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }

    // abort early if any missing
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    // RN0001: name content → MA0004
    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap();
    if !name_re.is_match(&payload.name) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0002: email format → MA0004
    let email_re = Regex::new(r"^[^@\s]+@[^@\s]+\.(com|br)$").unwrap();
    if payload.email.len() > 100 || !email_re.is_match(&payload.email) {
        errors.push(ValidationError {
            field: "email",
            code: "RN0002",
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
                field: "email",
                code: "RN0002",
                message: "E-mail já está cadastrado no sistema.".into(), // MA0002
            });
        }
    }

    // RN0003: password rules → MA0004
    let pwd = &payload.password;
    let is_too_short = pwd.len() < 8;
    let is_too_long = pwd.len() > 20;
    let is_all_digits = pwd.chars().all(|c| c.is_ascii_digit());
    let is_all_letters = pwd.chars().all(|c| c.is_alphabetic());
    let is_all_special = pwd.chars().all(|c| !c.is_alphanumeric());

    if is_too_short || is_too_long || is_all_digits || is_all_letters || is_all_special {
        errors.push(ValidationError {
            field: "password",
            code: "RN0003",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0004: CPF/CNPJ must be valid → MA0004
    let id = payload.cpf_cnpj.as_ref().unwrap();
    let id_digits_re = Regex::new(r"^\d{11}$|^\d{14}$").unwrap();
    let valid_id = if id_digits_re.is_match(id) {
        validate_cpf(id) || validate_cnpj(id)
    } else {
        false
    };
    if !valid_id {
        errors.push(ValidationError {
            field: "cpf_cnpj",
            code: "RN0004",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    if !errors.is_empty() {
        Err(HttpResponse::BadRequest().json(errors))
    } else {
        Ok(())
    }
}

fn validate_cpf(id: &str) -> bool {
    if id.len() != 11 || id.chars().all(|c| c == id.chars().next().unwrap()) {
        return false;
    }
    let digits: Vec<u8> = id.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let mut sum: u32 = 0;
    for i in 0..9 {
        sum += (digits[i] as u32) * (10 - i as u32);
    }
    let mut dv1 = (sum * 10) % 11;
    if dv1 == 10 {
        dv1 = 0;
    }
    if dv1 as u8 != digits[9] {
        return false;
    }
    sum = 0;
    for i in 0..10 {
        sum += (digits[i] as u32) * (11 - i as u32);
    }
    let mut dv2 = (sum * 10) % 11;
    if dv2 == 10 {
        dv2 = 0;
    }
    dv2 as u8 == digits[10]
}

fn validate_cnpj(id: &str) -> bool {
    if id.len() != 14 || id.chars().all(|c| c == id.chars().next().unwrap()) {
        return false;
    }
    let digits: Vec<u8> = id.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u8).collect();
    let weights1 = [5u32, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum: u32 = 0;
    for i in 0..12 {
        sum += (digits[i] as u32) * weights1[i];
    }
    let mut dv1 = sum % 11;
    dv1 = if dv1 < 2 { 0 } else { 11 - dv1 };
    if dv1 as u8 != digits[12] {
        return false;
    }
    let weights2 = [6u32, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    sum = 0;
    for i in 0..13 {
        sum += (digits[i] as u32) * weights2[i];
    }
    let mut dv2 = sum % 11;
    dv2 = if dv2 < 2 { 0 } else { 11 - dv2 };
    dv2 as u8 == digits[13]
}
