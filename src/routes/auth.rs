use crate::handlers::auth::login_user;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/login", web::post().to(login_user)));
}
