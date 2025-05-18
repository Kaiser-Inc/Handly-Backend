use crate::services::auth::verify_token;
use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse, Scope};

pub fn scope() -> Scope {
    web::scope("/protected").route("", web::get().to(protected))
}

async fn protected(req: HttpRequest) -> HttpResponse {
    // Bearer <token>
    let header = match req.headers().get(AUTHORIZATION) {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match header.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match verify_token(token, "access") {
        Some(claims) => HttpResponse::Ok().json(serde_json::json!({ "user_id": claims.sub })),
        None => HttpResponse::Unauthorized().finish(),
    }
}
