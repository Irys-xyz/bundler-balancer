use std::path::Path;

use actix_web::{HttpResponse, web::Data};
use log::info;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<
) -> actix_web::Result<HttpResponse> {

    for bundler in bundlers.into_iter() {
        // Create request builder, configure request and send
        let request = client
            .head(format!("{}/{}", bundler, ""))
            .send()
            .await;

        match request {
            Ok(req) => todo!(),
            Err(e) => {
                info!(format!("Error occurred while getting {} from {} - {}", ))
            }
        }
    }
    

    Ok(HttpResponse::NotFound().finish())
}