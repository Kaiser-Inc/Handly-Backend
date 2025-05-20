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

// Import generated Utoipa path definitions
use crate::handlers::auth::__path_login_user;
use crate::handlers::auth::__path_refresh_token;
use crate::handlers::protected::__path_get_profile;
use crate::handlers::protected::__path_upload_profile_pic;
use crate::handlers::services::__path_create_service;
use crate::handlers::services::__path_delete_service;
use crate::handlers::services::__path_get_service;
use crate::handlers::services::__path_list_services;
use crate::handlers::services::__path_update_service;
use crate::handlers::services::__path_upload_service_image;
use crate::handlers::users::__path_create_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        create_user,
        login_user,
        refresh_token,
        get_profile,
        upload_profile_pic,
        create_service,
        update_service,
        list_services,
        get_service,
        delete_service,
        upload_service_image
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
            crate::handlers::protected::ServiceInfo,
            crate::handlers::protected::Profile,
            crate::handlers::protected::ProfilePicResponse,
            crate::handlers::services::ImageResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoint"),
        (name = "users", description = "User operations"),
        (name = "auth", description = "Authentication operations"),
        (name = "protected", description = "Protected endpoints requiring authentication"),
        (name = "services", description = "Service operations")
    )
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "API health check")
    ),
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
            .service(Files::new("/static", "./uploads").show_files_listing())
            .service(Files::new("/uploads/services", "./uploads/services").show_files_listing())
            .service(health_check)
            .configure(routes::users::init)
            .configure(routes::auth::init)
            .configure(routes::protected::init)
            .configure(routes::services::init)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
