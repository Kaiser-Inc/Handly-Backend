use crate::handlers::providers::get_provider_profile;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/provider-profile/{cpf_cnpj}",
        web::get().to(get_provider_profile),
    );
}
