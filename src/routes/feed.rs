use crate::handlers::feed::get_feed;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/feed").route(web::get().to(get_feed)));
}
