use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    uptime: f32,
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    let res = IndexResponse {
        uptime: psutil::host::uptime().unwrap().as_secs_f32(),
    };
    Ok(HttpResponse::Ok().json(res))
}
