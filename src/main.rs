mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is up and running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .configure(routes::users::init)
            .configure(routes::auth::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
