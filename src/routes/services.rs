use actix_web::web;

use crate::handlers::services::{
    create_service, delete_service, get_service, list_services, update_service,
    upload_service_image,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/services")
            .route("", web::post().to(create_service))
            .route("/{id}", web::put().to(update_service))
            .route("", web::get().to(list_services))
            .route("/{id}", web::get().to(get_service))
            .route("/{id}", web::delete().to(delete_service))
            .route("/{id}/image", web::post().to(upload_service_image)),
    );
}
