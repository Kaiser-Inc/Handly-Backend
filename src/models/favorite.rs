use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct FavoriteEntry {
    pub target_type: String, // "service" | "provider"
    pub target_id: String,   // UUID ou CPF/CNPJ
}
