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

pub async fn validate_create_service_payload(payload: &CreateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if payload.categories.is_empty() {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0006",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap();
    if !name_re.is_match(payload.name.trim()) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(),
        });
    }

    let desc = payload.description.trim();
    if !desc.is_empty() {
        let len = desc.chars().count();
        if len < 10 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0007",
                message: "A descrição deve ter no mínimo 10 caracteres.".into(), // MA0022
            });
        } else if len > 300 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0007",
                message: "A descrição deve ter no máximo 300 caracteres.".into(), // MA0023
            });
        }
    }

    for cat in &payload.categories {
        if !CATEGORIES.contains(&cat.as_str()) {
            errors.push(ValidationError {
                field: "categories",
                code: "RN0008",
                message: "Um campo não foi preenchido corretamente.".into(),
            });
            break;
        }
    }
    if payload.categories.len() > 5 {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0009",
            message: "Um campo não foi preenchido corretamente.".into(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest().json(errors))
    }
}

pub async fn validate_update_service_payload(payload: &UpdateService) -> Result<(), HttpResponse> {
    let mut errors = Vec::new();

    if payload.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if payload.categories.is_empty() {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0006",
            message: "Preencha todos os campos obrigatórios.".into(),
        });
    }
    if !errors.is_empty() {
        return Err(HttpResponse::BadRequest().json(errors));
    }

    let name_re = Regex::new(r"^[A-Za-zÀ-ÖØ-öø-ÿ\s]{2,60}$").unwrap();
    if !name_re.is_match(payload.name.trim()) {
        errors.push(ValidationError {
            field: "name",
            code: "RN0001",
            message: "Um campo não foi preenchido corretamente.".into(),
        });
    }

    let desc = payload.description.trim();
    if !desc.is_empty() {
        let len = desc.chars().count();
        if len < 10 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0007",
                message: "A descrição deve ter no mínimo 10 caracteres.".into(), // MA0022
            });
        } else if len > 300 {
            errors.push(ValidationError {
                field: "description",
                code: "RN0007",
                message: "A descrição deve ter no máximo 300 caracteres.".into(), // MA0023
            });
        }
    }

    for cat in &payload.categories {
        if !CATEGORIES.contains(&cat.as_str()) {
            errors.push(ValidationError {
                field: "categories",
                code: "RN0008",
                message: "Um campo não foi preenchido corretamente.".into(),
            });
            break;
        }
    }
    if payload.categories.len() > 5 {
        errors.push(ValidationError {
            field: "categories",
            code: "RN0009",
            message: "Um campo não foi preenchido corretamente.".into(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest().json(errors))
    }
}
