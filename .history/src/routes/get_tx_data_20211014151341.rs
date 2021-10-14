use actix_web::{HttpResponse, web::{Data, Path}};
use log::info;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<(String,)>
) -> actix_web::Result<HttpResponse> {
    let (tx_id,) = path.into_inner();
    for bundler in bundlers.iter() {
        let url = format!("{}/{}", bundler, "");
        // Create request builder, configure request and send
        let request = client
            .head(url)
            .send()
            .await;

        match request {
            Ok(req) => {
                if req.status().is_success() {
                    info!("Found {} at {}", tx_id, bundler);
                    return Ok(HttpResponse::Ok()
                        .insert_header(("Location", url)).finish());
                } else {
                    info!("Not found {} at {}", tx_id, bundler);
                    continue;
                }
            },
            Err(e) => {
                info!("Error occurred while getting {} from {} - {}", tx_id, bundler, e);
                continue;
            }
        }
    }
    

    Ok(HttpResponse::NotFound().finish())
}