use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String, // "customer" | "provider"
}
