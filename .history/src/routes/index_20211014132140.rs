use actix_web::HttpResponse;

pub struct 

pub async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(value))
}