mod helper;
use actix_web::{test, web, App};
use handly_backend::routes::{auth, users};
use serde_json::json;

#[actix_web::test]
async fn customer_login_success_returns_tokens() {
    let pool = helper::setup_test_db().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init)
            .configure(auth::init),
    )
    .await;
    
    // create customer user
    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Bob",
                "email": "bob@test.dev",
                "password": "password123",
                "role": "customer",
                "cpf_cnpj": null
            }))
            .to_request(),
    )
    .await;
    
    // customer login
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "bob@test.dev",
                "password": "password123"
            }))
            .to_request(),
    )
    .await;
    
    assert_eq!(resp.status(), 200);
    
    // verify tokens
    let body = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(v.get("access_token").is_some(), "access token missing");
    assert!(v.get("refresh_token").is_some(), "refresh token missing");
}

#[actix_web::test]
async fn provider_login_success_returns_tokens() {
    let pool = helper::setup_test_db().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init)
            .configure(auth::init),
    )
    .await;
    
    // create provider user
    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Services XYZ",
                "email": "contact@xyz.com",
                "password": "password123",
                "role": "provider",
                "cpf_cnpj": "12.345.678/0001-99"
            }))
            .to_request(),
    )
    .await;
    
    // provider login
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "contact@xyz.com",
                "password": "password123"
            }))
            .to_request(),
    )
    .await;
    
    assert_eq!(resp.status(), 200);
    
    // verify tokens
    let body = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(v.get("access_token").is_some(), "access token missing");
    assert!(v.get("refresh_token").is_some(), "refresh token missing");
}