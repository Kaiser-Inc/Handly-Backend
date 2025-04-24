use sqlx::{PgPool, Connection, Executor, PgConnection};
use std::env;
use dotenvy::dotenv;
use uuid::Uuid;

// create a temporary database
pub async fn setup_test_db() -> PgPool {
    dotenv().ok();
    
    let base = env::var("DATABASE_URL_TEST")
        .expect("Set DATABASE_URL_TEST in .env or shell");
    let db_name = format!("test_{}", Uuid::new_v4());

    let mut conn = PgConnection::connect(&base).await.unwrap();
    conn.execute(format!(r#"CREATE DATABASE "{}""#, db_name).as_str())
        .await
        .unwrap();

    let mut url: url::Url = base.parse().unwrap();
    url.set_path(&db_name);
    let pool = PgPool::connect(url.as_str()).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
