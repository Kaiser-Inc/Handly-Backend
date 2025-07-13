use crate::handlers::ratings::{create_rating, list_ratings};
use crate::handlers::services::{
    create_service, delete_service, get_service, get_service_image, list_services, update_service,
    upload_service_image,
};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/services")
            .route("", web::post().to(create_service))
            .route("/{id}", web::put().to(update_service))
            .route("", web::get().to(list_services))
            .route("/{id}", web::get().to(get_service))
            .route("/{id}", web::delete().to(delete_service))
            .route("/{id}/image", web::post().to(upload_service_image))
            .route("/{id}/image", web::get().to(get_service_image))
            .route("/{id}/ratings", web::post().to(create_rating))
            .route("/{id}/ratings", web::get().to(list_ratings)),
    );
}
