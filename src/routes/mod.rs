pub mod auth;
pub mod protected;
pub mod users;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(users::init)
        .configure(auth::init)
        .service(protected::scope()); //  GET /protected
}
