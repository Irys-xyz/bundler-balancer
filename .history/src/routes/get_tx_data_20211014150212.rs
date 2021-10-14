use actix_web::{HttpResponse, web::Data};

pub async fn get_tx_data(bundlers: Data<Vec<String>>) -> actix_web::Result<HttpResponse> {
    let client = awc::Client::new();

    // Create request builder, configure request and send
    let request = client
        .get("https://www.rust-lang.org/")
        .append_header(("User-Agent", "Actix-web"));

    Ok(HttpResponse::NotFound().finish())
}