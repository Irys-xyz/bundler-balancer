use actix_web::{HttpResponse, web::{Data, Path}};
use log::info;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<(String,)>
) -> actix_web::Result<HttpResponse> {
    let (tx_id,) = path.into_inner();
    for bundler in bundlers.into_iter() {
        // Create request builder, configure request and send
        let request = client
            .head(format!("{}/{}", bundler, ""))
            .send()
            .await;

        match request {
            Ok(req) => todo!(),
            Err(e) => {
                info!(format!("Error occurred while getting {} from {} -", tx_id, bundler))
            }
        }
    }
    

    Ok(HttpResponse::NotFound().finish())
}