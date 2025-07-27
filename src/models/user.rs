use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub phone: Option<String>,
    #[schema(value_type = Vec<crate::models::favorite::FavoriteEntry>)]
    pub favorites: Json<Vec<crate::models::favorite::FavoriteEntry>>,
}
