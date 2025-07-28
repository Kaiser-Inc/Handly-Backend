use crate::handlers::reports::{report_service, report_user};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reports")
            .route("/service/{id}", web::post().to(report_service))
            .route("/user/{cpf_cnpj}", web::post().to(report_user)),
    );
}
