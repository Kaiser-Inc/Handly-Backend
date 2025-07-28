mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod validations;

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::auth::{__path_login_user, __path_refresh_token};
use crate::handlers::categories::__path_get_categories;
use crate::handlers::favorites::{__path_list_favorites, __path_toggle_favorite};
use crate::handlers::feed::__path_get_feed;
use crate::handlers::protected::{
    __path_get_profile, __path_get_profile_pic, __path_update_profile, __path_upload_profile_pic,
};
use crate::handlers::provider_ratings::{
    __path_create_rating as __path_create_provider_rating,
    __path_list_ratings as __path_list_provider_ratings,
};
use crate::handlers::ratings::{__path_create_rating, __path_list_ratings};
use crate::handlers::reports::{__path_report_service, __path_report_user};
use crate::handlers::services::{
    __path_create_service, __path_delete_service, __path_get_publisher_profile_pic,
    __path_get_service, __path_get_service_image, __path_list_services, __path_update_service,
    __path_upload_service_image,
};
use crate::handlers::users::__path_create_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        create_user,
        login_user,
        refresh_token,
        get_profile,
        update_profile,
        upload_profile_pic,
        get_profile_pic,
        create_service,
        update_service,
        list_services,
        get_service,
        delete_service,
        upload_service_image,
        get_service_image,
        get_publisher_profile_pic,
        get_feed,
        get_categories,
        toggle_favorite,
        list_favorites,
        create_rating,
        list_ratings,
        create_provider_rating,
        list_provider_ratings,
        report_service,
        report_user
    ),
    components(
        schemas(
            crate::models::user::User,
            crate::handlers::users::CreateUser,
            crate::handlers::auth::LoginRequest,
            crate::handlers::auth::RefreshRequest,
            crate::models::service::Service,
            crate::handlers::services::CreateService,
            crate::handlers::services::UpdateService,
            crate::handlers::protected::Profile,
            crate::handlers::protected::UpdateProfile,
            crate::handlers::protected::ProfilePicResponse,
            crate::handlers::services::ImageResponse,
            crate::handlers::feed::FeedItem,
            crate::handlers::categories::CategoriesResponse,
            crate::models::favorite::FavoriteEntry,
            crate::models::rating::ServiceRating,
            crate::models::provider_rating::ProviderRating
        )
    ),
    tags(
        (name = "health", description = "Health check endpoint"),
        (name = "users", description = "User operations"),
        (name = "auth", description = "Authentication operations"),
        (name = "protected", description = "Protected endpoints requiring authentication"),
        (name = "services", description = "Service operations"),
        (name = "favorites", description = "Favorite services & providers"),
        (name = "providers", description = "Provider ratings")
    )
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "API health check")),
    tag = "health"
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is up and running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                Files::new("/uploads", "./uploads")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
            .service(health_check)
            .configure(routes::users::init)
            .configure(routes::auth::init)
            .configure(routes::protected::init)
            .configure(routes::provider_ratings::init)
            .configure(routes::favorites::init)
            .configure(routes::services::init)
            .configure(routes::feed::init)
            .configure(routes::reports::init)
            .configure(routes::categories::init)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
