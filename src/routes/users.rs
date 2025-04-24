use crate::handlers::users::create_user;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::post().to(create_user)));
}
