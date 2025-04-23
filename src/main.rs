use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is up and running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
