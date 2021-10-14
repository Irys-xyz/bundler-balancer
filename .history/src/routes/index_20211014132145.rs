use actix_web::HttpResponse;

pub struct IndexResponse {
    
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(value))
}