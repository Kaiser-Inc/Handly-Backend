use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::report::ReportReason;
use crate::services::auth::verify_token;
use crate::validations::{validate_report_payload, ValidationError};

#[derive(Deserialize, ToSchema)]
pub struct ReportPayload {
    pub reason: Option<ReportReason>,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
struct ApiMessage<'a> {
    code: &'a str,
    message: &'a str,
}

#[utoipa::path(
    post,
    path = "/reports/service/{id}",
    security(("bearerAuth" = [])),
    params(("id" = String, Path, description = "Service ID")),
    request_body = ReportPayload,
    responses(
        (status = 201, description = "Report created", body = ApiMessage),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "reports"
)]
pub async fn report_service(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
    payload: web::Json<ReportPayload>,
) -> HttpResponse {
    let reporter = match auth_claim(&req) {
        Ok(pk) => pk,
        Err(r) => return r,
    };

    if let Err(resp) = validate_report_payload(&payload) {
        return resp;
    }

    let sid = path.into_inner().to_string();
    if let Err(e) = insert_report(&pool, &reporter, "service", &sid, &payload).await {
        return e;
    }

    HttpResponse::Created().json(ApiMessage {
        code: "MA0011",
        message: "Sua denúncia foi enviada.",
    })
}

#[utoipa::path(
    post,
    path = "/reports/user/{cpf_cnpj}",
    security(("bearerAuth" = [])),
    params(("cpf_cnpj" = String, Path, description = "Target CPF/CNPJ")),
    request_body = ReportPayload,
    responses(
        (status = 201, description = "Report created", body = ApiMessage),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "reports"
)]
pub async fn report_user(
    req: HttpRequest,
    path: web::Path<String>,
    pool: web::Data<PgPool>,
    payload: web::Json<ReportPayload>,
) -> HttpResponse {
    let reporter = match auth_claim(&req) {
        Ok(pk) => pk,
        Err(r) => return r,
    };

    if let Err(resp) = validate_report_payload(&payload) {
        return resp;
    }

    let target = path.into_inner();
    if reporter == target {
        return HttpResponse::BadRequest().json([ValidationError {
            field: "target",
            code: "RN0010",
            message: "Algo deu errado, tente novamente.".into(),
        }]);
    }

    if let Err(e) = insert_report(&pool, &reporter, "user", &target, &payload).await {
        return e;
    }

    HttpResponse::Created().json(ApiMessage {
        code: "MA0011",
        message: "Sua denúncia foi enviada.",
    })
}

fn auth_claim(req: &HttpRequest) -> Result<String, HttpResponse> {
    let tok = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    verify_token(tok, "access")
        .map(|c| c.sub)
        .ok_or_else(|| HttpResponse::Unauthorized().finish())
}

async fn insert_report(
    pool: &PgPool,
    reporter_key: &str,
    target_type: &str,
    target_id: &str,
    p: &ReportPayload,
) -> Result<(), HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO reports (reporter_key, target_type, target_id, reason_code, description)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        reporter_key,
        target_type,
        target_id,
        p.reason.unwrap() as ReportReason,
        p.description
    )
    .execute(pool)
    .await
    .map_err(|_| {
        HttpResponse::InternalServerError().json([ValidationError {
            field: "auth",
            code: "MA0001",
            message: "Algo deu errado, tente novamente.".into(),
        }])
    })?;
    Ok(())
}
