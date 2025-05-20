// tests/create_user.rs
mod helper;

use actix_web::{test, web, App};
use handly_backend::routes::users;
use serde_json::json;

#[actix_web::test]
async fn create_customer_user_returns_201() {
    let pool = helper::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init),
    )
    .await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Alice",
                "email": "alice@example.com",
                "password": "Password1",
                "role": "customer",
                "cpf_cnpj": "12345678900"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 201);
}

#[actix_web::test]
async fn create_provider_user_returns_201() {
    let pool = helper::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init),
    )
    .await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Services XYZ",
                "email": "contact@xyz.com",
                "password": "Password1",
                "role": "provider",
                "cpf_cnpj": "12345678000199"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 201);
}
