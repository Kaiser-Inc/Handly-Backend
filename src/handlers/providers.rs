use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PublicProfile {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub role: String,
    #[schema(value_type = Option<String>)]
    pub phone: Option<String>,
    #[schema(value_type = Option<String>)]
    pub profile_pic: Option<String>,
}

#[utoipa::path(
    get,
    path = "/provider-profile/{cpf_cnpj}",
    params(
        ("cpf_cnpj" = String, Path, description = "CPF ou CNPJ do usuário")
    ),
    responses(
        (status = 200, description = "Public profile", body = PublicProfile),
        (status = 404, description = "User not found")
    ),
    tag = "providers"
)]
pub async fn get_provider_profile(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let key = path.into_inner();

    match sqlx::query_as!(
        PublicProfile,
        r#"
        SELECT cpf_cnpj,
               name,
               email,
               role,
               phone       AS "phone?: String",
               profile_pic AS "profile_pic?: String"
          FROM users
         WHERE cpf_cnpj = $1
        "#,
        key
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(p)) => HttpResponse::Ok().json(p),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
