mod helper;

use actix_web::{test, web, App};
use handly_backend::routes::{auth, users};
use serde_json::json;

#[actix_web::test]
async fn login_success_returns_tokens() {
    let pool = helper::setup_test_db().await;

    // build the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init)
            .configure(auth::init),
    )
    .await;

    // create user
    let _ = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/users")
            .set_json(json!({
                "name": "Bob",
                "email": "bob@test.dev",
                "password": "pwd"
            }))
            .to_request(),
    )
    .await;

    // login
    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "bob@test.dev",
                "password": "pwd"
            }))
            .to_request(),
    )
    .await;

    assert_eq!(resp.status(), 200);

    // must return both tokens
    let body = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(v.get("access_token").is_some(), "access token missing");
    assert!(v.get("refresh_token").is_some(), "refresh token missing");
}
