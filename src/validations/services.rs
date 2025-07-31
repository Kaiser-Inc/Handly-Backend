use crate::handlers::services::{CreateService, UpdateService};
use crate::validations::ValidationError;
use actix_web::HttpResponse;
use regex::Regex;

pub const CATEGORIES: &[&str] = &[
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
    "desenvolvedor de software",
    "analista de sistemas",
    "suporte técnico",
    "administrador de redes",
    "consultor de TI",
    "técnico em hardware",
    "web designer",
    "UX/UI designer",
    "especialista em segurança da informação",
    "programador mobile",
    "técnico em telecomunicações",
    "digital influencer",
    "redator freelancer",
    "assistente virtual",
    "montador e reparador de computadores",
    "técnico em manutenção de impressoras",
    "técnico em automação residencial",
    "técnico em manutenção de drones",
    "produtor de conteúdo digital",
    "instrutor de informática básica",
    "editor de vídeo",
    "animador digital",
    "tester de software",
    "gerente de projetos",
    "arquiteto de software",
    "administrador de banco de dados",
    "técnico em redes de fibra óptica",
    "instalador de sistemas solares",
    "técnico em energias renováveis",
    "eletricista industrial",
    "técnico em manutenção industrial",
    "operador de máquinas",
    "ajudante geral",
    "soldador",
    "técnico em refrigeração",
    "consultor financeiro",
    "corretor de imóveis",
    "tradutor",
    "intérprete",
    "agente de viagens",
    "coach",
    "terapeuta ocupacional",
    "nutricionista",
    "auxiliar administrativo",
    "recepcionista",
    "assistente social",
];

/// Validate CreateService payload according to [RF05] RN0001–RN0009 business rules,
/// mapping to MA0003 (campos obrigatórios) and MA0004 (preenchimento incorreto).
pub async fn validate_create_service_payload(payload: &CreateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // missing mandatory fields → MA0003
    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.description.trim().is_empty() {
        errors.push(ValidationError {
            field: "description",
            code: "RN0005",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.categories.is_empty() {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0006",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }

    // abort early if any missing
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    // RN0001: name content → MA0004
    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap();
    if !name_re.is_match(payload.name.trim()) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0007: description length → MA0004
    if payload.description.chars().count() < 10 || payload.description.chars().count() > 300 {
        errors.push(ValidationError {
            field: "description",
            code: "RN0007",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0008: category values → MA0004
    for cat in &payload.categories {
        if !CATEGORIES.contains(&cat.as_str()) {
            errors.push(ValidationError {
                field: "categories",
                code: "RN0008",
                message: "Um campo não foi preenchido corretamente.".into(), // MA0004
            });
            break;
        }
    }

    // RN0009: at most 5 categories → MA0004
    if payload.categories.len() > 5 {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0009",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}

/// Validate UpdateService payload according to [RF05] RN0001–RN0009 business rules,
/// mapping to MA0003 (campos obrigatórios) and MA0004 (preenchimento incorreto).
pub async fn validate_update_service_payload(payload: &UpdateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    // missing mandatory fields → MA0003
    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.description.trim().is_empty() {
        errors.push(ValidationError {
            field: "description",
            code: "RN0005",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }
    if payload.categories.is_empty() {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0006",
            message: "Preencha todos os campos obrigatórios.".into(), // MA0003
        });
    }

    // abort early if any missing
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    // RN0001: name content → MA0004
    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap();
    if !name_re.is_match(payload.name.trim()) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0007: description length → MA0004
    if payload.description.chars().count() > 300 {
        errors.push(ValidationError {
            field: "description",
            code: "RN0007",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    // RN0008: category values → MA0004
    for cat in &payload.categories {
        if !CATEGORIES.contains(&cat.as_str()) {
            errors.push(ValidationError {
                field: "categories",
                code: "RN0008",
                message: "Um campo não foi preenchido corretamente.".into(), // MA0004
            });
            break;
        }
    }

    // RN0009: at most 5 categories → MA0004
    if payload.categories.len() > 5 {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0009",
            message: "Um campo não foi preenchido corretamente.".into(), // MA0004
        });
    }

    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }
    Ok(())
}
