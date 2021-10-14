use actix_web::HttpResponse;

pub struct IndexResponse {
    uptime: u6
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(value))
}