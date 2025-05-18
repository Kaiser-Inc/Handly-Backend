use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String, // "customer" | "provider"
}
