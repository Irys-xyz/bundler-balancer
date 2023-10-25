use std::time::Duration;

use actix_web::{
    web::{Data, Path},
    HttpMessage, HttpResponse,
};
use futures::{StreamExt, stream::FuturesOrdered};
use log::info;
use actix_web::Either;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Either<Path<(String, String)>, Path<(String)>>,
) -> actix_web::Result<HttpResponse> {
    let (tx_id, field) = match path {
        Either::Left(s) => s.into_inner(),
        Either::Right(s) => (s.into_inner(), "data".to_string())
    };

    let futures = bundlers.iter().map(|b| {
        let tx_id: String = tx_id.clone();
        let client = client.clone();

        debug!("Checking {} for tx id {}", b, tx_id);

        let url = format!("{}/tx/{}/{}", b, tx_id, field);
            // Create request builder, configure request and send
            async move { (b, url.clone(), client.head(url).timeout(Duration::from_millis(30 * 1000)).send().await) }

        });

    let s = futures::stream::iter(futures);

    let x = s.buffer_unordered(2).skip_while(|(b, _, r)| {
        match r {
                Ok(req) => {
                    if req.status().is_success() {
                        info!("Found {} at {}", tx_id, b);
                    debug!("Headers {:?}", req.headers());
                        std::future::ready(false)
                    } else {
                        debug!("Not success getting from {}", b);

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    info!(
                        "Error occurred while getting {} from {} - {}",
                        tx_id, b, e
                    );
                    std::future::ready(true)
                }
            }
    }).next().await;

    match x {
        Some((_, url, r)) => {
            let r = r.unwrap();
            if let Some(h) = r.headers().get("Content-Length") {
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
        },
        None => Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
    }
}

pub async fn get_tx_data_manifest(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Either<Path<(String, String)>, Path<(String)>>,
) -> actix_web::Result<HttpResponse> {
    let (tx_id, pathh) = match path {
        Either::Left(s) => {
            let i = s.into_inner();
            (i.0, "/".to_owned() + &i.1)
        },
        Either::Right(s) => (s.into_inner(), "".to_owned())
    };
   
    let futures = bundlers.iter().map(|b| {
        let tx_id: String = tx_id.clone();
        let client = client.clone();

        debug!("Checking {} for tx id {}", b, tx_id);

        let url = format!("{}/{}{}", b, tx_id, pathh);
            // Create request builder, configure request and send
            async move { (b, url.clone(), client.head(url).timeout(Duration::from_millis(30 * 1000)).send().await) }

        });

    let s = futures::stream::iter(futures);

    let x = s.buffer_unordered(2).skip_while(|(b, _, r)| {
        match r {
                Ok(req) => {
                    if req.status().is_success() {
                        info!("Found {} at {}", tx_id, b);
                    debug!("Headers {:?}", req.headers());
                        std::future::ready(false)
                    } else {
                        debug!("Not success getting from {}", b);

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    info!(
                        "Error occurred while getting {} from {} - {}",
                        tx_id, b, e
                    );
                    std::future::ready(true)
                }
            }
    }).next().await;

    match x {
        Some((_, url, r)) => {
            let r = r.unwrap();
            if let Some(h) = r.headers().get("Content-Length") {
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
        },
        None => Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
    }
}


pub async fn get_tx_meta(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
    path: Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (tx_id,) = path.into_inner();

    let futures = bundlers.iter().map(|b| {
        let tx_id: String = tx_id.clone();
        let client = client.clone();

        debug!("Checking {} for tx id {}", b, tx_id);

            let url = format!("{}/tx/{}", b, tx_id);
            // Create request builder, configure request and send
            async move { (b, url.clone(), client.head(url).timeout(Duration::from_millis(30 * 1000)).send().await) }

        });

    let s = futures::stream::iter(futures);

    let x = s.buffer_unordered(2).skip_while(|(b, _, r)| {
        match r {
                Ok(req) => {
                    if req.status().is_success() {
                        info!("Found {} at {}", tx_id, b);
                    debug!("Headers {:?}", req.headers());
                        std::future::ready(false)
                    } else {
                        debug!("Not success getting from {}", b);

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    info!(
                        "Error occurred while getting {} from {} - {}",
                        tx_id, b, e
                    );
                    std::future::ready(true)
                }
            }
    }).next().await;

    match x {
        Some((_, url, r)) => {
            let r = r.unwrap();
            if let Some(h) = r.headers().get("Content-Length") {
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
        },
        None => Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
    }

}
