use std::time::Duration;
use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct IndexResponse {
    uptime: Duration
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    let res = IndexResponse{
        uptime: psutil::host::uptime().unwrap().as_secs()
    };
    Ok(HttpResponse::Ok()
        .json(res))
}