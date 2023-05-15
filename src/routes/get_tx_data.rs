use actix_web::{
    web::{Data, Path},
    HttpMessage, HttpResponse,
};
use log::info;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<(String, String)>,
) -> actix_web::Result<HttpResponse> {
    let (tx_id, field) = path.into_inner();
    for bundler in bundlers.iter() {
        let url = format!("{}/tx/{}/{}", bundler, tx_id, field);
        // Create request builder, configure request and send
        let request = client.head(&url).send().await;

        match request {
            Ok(req) => {
                if req.status().is_success() {
                    info!("Found {} at {}", tx_id, bundler);
                    debug!("Headers {:?}", req.headers());
                    if let Some(h) = req.headers().get("Content-Length") {
                        return Ok(HttpResponse::Found()
                            .insert_header(("Content-Length", h))
                            .insert_header(("Location", url))
                            .insert_header(("Cache-Control", "max-age=86400"))
                            .finish());
                    } else {
                        return Ok(HttpResponse::Found()
                            .insert_header(("Location", url))
                            .insert_header(("Cache-Control", "max-age=86400"))
                            .finish());
                    }
                }
                if req.status().is_redirection() {
                    info!("Found {} at {} - redirecting", tx_id, bundler);
                    let fallback_redir = format!("http://arweave.net/{}", tx_id);
                    return Ok(HttpResponse::PermanentRedirect()
                        .insert_header((
                            "Location",
                            match req.headers().get("Location") {
                                Some(v) => v.to_str().unwrap(),
                                None => fallback_redir.as_str(),
                            },
                        ))
                        .finish());
                } else {
                    info!("Not found {} at {}", tx_id, bundler);
                    continue;
                }
            }
            Err(e) => {
                info!(
                    "Error occurred while getting {} from {} - {}",
                    tx_id, bundler, e
                );
                continue;
            }
        }
    }

    Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
}

pub async fn get_tx_meta(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (tx_id,) = path.into_inner();
    for bundler in bundlers.iter() {
        let url = format!("{}/tx/{}", bundler, tx_id);
        // Create request builder, configure request and send
        let request = client.head(&url).send().await;

        match request {
            Ok(req) => {
                if req.status().is_success() {
                    info!("Found {} at {}", tx_id, bundler);
                    debug!("Headers {:?}", req.headers());
                    if let Some(h) = req.headers().get("Content-Length") {
                        return Ok(HttpResponse::Found()
                            .insert_header(("Content-Length", h))
                            .insert_header(("Location", url))
                            .insert_header(("Cache-Control", "max-age=86400"))
                            .finish());
                    } else {
                        return Ok(HttpResponse::Found()
                            .insert_header(("Location", url))
                            .insert_header(("Cache-Control", "max-age=86400"))
                            .finish());
                    }
                } else {
                    info!("Not found {} at {}", tx_id, bundler);
                    continue;
                }
            }
            Err(e) => {
                info!(
                    "Error occurred while getting {} from {} - {}",
                    tx_id, bundler, e
                );
                continue;
            }
        }
    }

    Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
}
