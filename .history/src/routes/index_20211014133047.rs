use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct IndexResponse {
    uptime: u64
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    let res = IndexResponse{
        uptime: psutil::host::uptime().unwrap().as_secs_f32()
    };
    Ok(HttpResponse::Ok()
        .json(res))
}