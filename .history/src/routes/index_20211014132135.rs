use actix_web::HttpResponse;

struct 

pub async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(value))
}