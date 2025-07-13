use crate::handlers::favorites::{list_favorites, toggle_favorite};
use actix_web::web::{self, ServiceConfig};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/favorites")
            .route(web::post().to(toggle_favorite))
            .route(web::get().to(list_favorites)),
    );
}
