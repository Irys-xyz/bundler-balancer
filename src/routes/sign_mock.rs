use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    uptime: f32,
}

pub async fn sign_mock() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("some signature".as_bytes()))
}
