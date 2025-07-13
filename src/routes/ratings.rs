use crate::handlers::ratings::{create_rating, list_ratings};
use actix_web::web::{self, ServiceConfig};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/services")
            .route("/{id}/ratings", web::post().to(create_rating))
            .route("/{id}/ratings", web::get().to(list_ratings)),
    );
}
