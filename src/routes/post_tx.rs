use std::cmp::Ordering;

use actix_web::{web::Data, HttpResponse};
use log::{info, trace};

pub async fn post_tx(
    bundlers: Data<Vec<String>>,
    client: Data<awc::Client>,
) -> actix_web::Result<HttpResponse> {
    let mut balances = Vec::with_capacity(bundlers.len());
    for bundler in bundlers.iter() {
        let url = format!("{}/tx", bundler);
        // Get balance
        let request = client.head(&url).send().await;

        trace!("Client has {} balance with {}", 1, bundler);
        balances.push(1);
    }

    let (index_of_max, max) = balances
        .into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    let max_bundler = bundlers.get(index_of_max).unwrap();

    trace!("Client has max balance {} with {}", max, max_bundler);

    if max == 0 {
        return Ok(HttpResponse::PaymentRequired().finish());
    };

    Ok(HttpResponse::Found()
        .insert_header(("Location", max_bundler.as_str()))
        .finish())
}
