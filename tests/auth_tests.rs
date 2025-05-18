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
                "email": "alice@test.dev",
                "password": "123",
                "role": "customer",
                "cpf_cnpj": "123.456.789-00"
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
                "password": "pwd",
                "role": "provider",
                "cpf_cnpj": "12.345.678/0001-99"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 201);
}
