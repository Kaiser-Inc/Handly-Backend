mod helper;

use actix_web::{test, web, App};
use handly_backend::routes::{users, auth};
use serde_json::json;

#[actix_web::test]
async fn login_with_valid_credentials_returns_200() {
    let pool = helper::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init)
            .configure(auth::init),
    )
    .await;

    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Bob",
                "email": "bob@example.com",
                "password": "Password1",
                "role": "customer",
                "cpf_cnpj": "12345678900"
            }))
            .to_request(),
    )
    .await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "bob@example.com",
                "password": "Password1"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn login_with_invalid_credentials_returns_401() {
    let pool = helper::setup_test_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init)
            .configure(auth::init),
    )
    .await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "notfound@example.com",
                "password": "wrong"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 401);
}
