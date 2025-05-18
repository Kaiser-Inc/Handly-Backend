use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Result};
use futures_util::StreamExt;
use sanitize_filename::sanitize;
use uuid::Uuid;
use std::io::Write;

#[post("/profile/upload")]
pub async fn upload_profile_pic(
    mut payload: Multipart,
) -> Result<HttpResponse> {
    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_type = field.content_disposition();
        if let Some(filename) = content_type.get_filename() {
            let safe = sanitize(filename);
            let uuid = Uuid::new_v4();
            let filepath = format!("./uploads/{}_{}", uuid, safe);
            let mut f = web::block(|| std::fs::File::create(&filepath)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }
            return Ok(HttpResponse::Ok().json({ "url"; format!("/static/{}", &filepath[2..]) }));
        }
    }
    Ok(HttpResponse::BadRequest().finish())
}
