use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct IndexResponse {
    uptime: isize
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    let res = IndexResponse{
        uptime: psutil::system::uptime()
    };
    Ok(HttpResponse::Ok()
        .json(res))
}