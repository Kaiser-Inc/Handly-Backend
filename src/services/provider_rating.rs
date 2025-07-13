use crate::models::provider_rating::ProviderRating;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn add_rating(
    pool: &PgPool,
    provider_id: &str,
    user_id: &str,
    stars: i16,
    comment: Option<String>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO provider_ratings (id, provider_id, user_id, stars, comment)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (provider_id, user_id)
        DO UPDATE SET stars = EXCLUDED.stars,
                      comment = EXCLUDED.comment,
                      created_at = NOW()
        "#,
        Uuid::new_v4(),
        provider_id,
        user_id,
        stars,
        comment
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_by_provider(
    pool: &PgPool,
    provider_id: &str,
) -> sqlx::Result<Vec<ProviderRating>> {
    sqlx::query_as!(
        ProviderRating,
        r#"
        SELECT id, provider_id, user_id, stars, comment, created_at
          FROM provider_ratings
         WHERE provider_id = $1
         ORDER BY created_at DESC
        "#,
        provider_id
    )
    .fetch_all(pool)
    .await
}
