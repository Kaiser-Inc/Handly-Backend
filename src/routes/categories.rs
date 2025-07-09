use crate::handlers::categories::get_categories;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_categories);
}
