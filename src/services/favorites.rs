use crate::models::favorite::FavoriteEntry;
use sqlx::{types::Json, PgPool, Result};

pub async fn list(pool: &PgPool, uid: &str) -> Result<Vec<FavoriteEntry>> {
    let rec = sqlx::query!(
        r#"SELECT favorites as "favorites!: Json<Vec<FavoriteEntry>>"
            FROM users WHERE cpf_cnpj = $1"#,
        uid
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.favorites.0)
}

pub async fn save(pool: &PgPool, uid: &str, favs: &[FavoriteEntry]) -> Result<()> {
    sqlx::query!(
        r#"UPDATE users SET favorites = $1 WHERE cpf_cnpj = $2"#,
        Json(favs.to_vec()) as _,
        uid
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn toggle(pool: &PgPool, uid: &str, entry: FavoriteEntry) -> Result<bool> {
    let mut favs = list(pool, uid).await?;
    if let Some(i) = favs.iter().position(|e| *e == entry) {
        favs.remove(i);
        save(pool, uid, &favs).await?;
        Ok(false) // removed
    } else {
        favs.push(entry);
        save(pool, uid, &favs).await?;
        Ok(true) // added
    }
}
