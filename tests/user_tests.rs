mod helper;

use actix_web::{test, web, App};
use handly_backend::routes::users;
use serde_json::json;

#[actix_web::test]
async fn create_user_returns_201() {
    // fresh database for each run
    let pool = helper::setup_test_db().await;

    // build the test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init),
    )
    .await;

    // call POST /users
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(json!({
            "name": "Alice",
            "email": "alice@test.dev",
            "password": "123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}
