use actix_web::{HttpResponse, web::Data};

pub async fn get_tx_data(bundlers: Data<Vec<String>>, client: Data<awc::Client>) -> actix_web::Result<HttpResponse> {

    for bundler in bundlers {
        bundler.
    }
    // Create request builder, configure request and send
    let request = client
        .head("https://www.rust-lang.org/")
        .append_header(("User-Agent", "Actix-web"));

    Ok(HttpResponse::NotFound().finish())
}