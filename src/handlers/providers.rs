use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::models::service::Service;

#[derive(Serialize, ToSchema)]
pub struct ProviderWithServices {
    pub cpf_cnpj: String,
    pub name: String,
    pub email: String,
    pub role: String,
    #[schema(value_type = Option<String>)]
    pub phone: Option<String>,
    #[schema(value_type = Option<String>)]
    pub profile_pic: Option<String>,
    pub services: Vec<Service>,
}

#[utoipa::path(
    get,
    path = "/provider-profile/{cpf_cnpj}",
    params(("cpf_cnpj" = String, Path, description = "CPF ou CNPJ do usuário")),
    responses(
        (status = 200, description = "Public profile + services", body = ProviderWithServices),
        (status = 404, description = "User not found")
    ),
    tag = "providers"
)]
pub async fn get_provider_profile(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let key = path.into_inner();

    // basic data
    let profile = match sqlx::query!(
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
        Ok(Some(p)) => p,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let services: Vec<Service> = match sqlx::query_as!(
        Service,
        r#"
        SELECT id, provider_key, categories, name, description, image, created_at, updated_at
          FROM services
         WHERE provider_key = $1
      ORDER BY created_at DESC
        "#,
        profile.cpf_cnpj
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(ProviderWithServices {
        cpf_cnpj: profile.cpf_cnpj,
        name: profile.name,
        email: profile.email,
        role: profile.role,
        phone: profile.phone,
        profile_pic: profile.profile_pic,
        services,
    })
}
