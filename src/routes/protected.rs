use crate::handlers::protected::get_profile;
use crate::services::auth::verify_token;
use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};
use serde_json::json;

async fn protected(req: HttpRequest) -> HttpResponse {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");
    match verify_token(token, "access") {
        Some(claims) => HttpResponse::Ok().json(json!({ "user_key": claims.sub })),
        None => HttpResponse::Unauthorized().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/protected")
            .route("", web::get().to(protected))
            // aqui registramos GET /protected/profile
            .route("/profile", web::get().to(get_profile)),
    );
}
