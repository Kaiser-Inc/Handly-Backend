use crate::models::rating::ServiceRating;
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub async fn add_rating(
    pool: &PgPool,
    service_id: Uuid,
    user_id: &str,
    stars: i16,
    comment: Option<String>,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO service_ratings (id, service_id, user_id, stars, comment)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (service_id, user_id)
        DO UPDATE SET stars = EXCLUDED.stars,
                      comment = EXCLUDED.comment,
                      created_at = NOW()
        "#,
        Uuid::new_v4(),
        service_id,
        user_id,
        stars,
        comment
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_by_service(pool: &PgPool, service_id: Uuid) -> Result<Vec<ServiceRating>> {
    sqlx::query_as!(
        ServiceRating,
        r#"
        SELECT id, service_id, user_id, stars, comment, created_at
          FROM service_ratings
         WHERE service_id = $1
         ORDER BY created_at DESC
        "#,
        service_id
    )
    .fetch_all(pool)
    .await
}
