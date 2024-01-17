use std::time::Duration;

use actix_web::{
    web::{Data, Path},
    HttpMessage, HttpResponse,
};
use futures::{StreamExt, stream::FuturesOrdered};
use log::info;
use actix_web::Either;
use reqwest_middleware::ClientWithMiddleware;
use serde_derive::Deserialize;
use serde_json::json;

pub async fn get_tx_data(
    bundlers: Data<Vec<String>>,
    client: Data<ClientWithMiddleware>,
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
                        debug!("Not success getting from {} - {}", b, req.status());

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    error!(
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
    client: Data<ClientWithMiddleware>,
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
                        debug!("Not success getting from {} - {}", b, req.status());

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    error!(
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
    client: Data<ClientWithMiddleware>,
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
                        debug!("Not success getting from {} - {}", b, req.status());

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    error!(
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

#[derive(Deserialize)]
struct GQLResponse {
    data: GQL
}

#[derive(Deserialize)]
struct GQL {
    transactions: GQLTransaction
}

impl Default for GQL {
    fn default() -> Self {
        Self { transactions: GQLTransaction { edges: Vec::new() } }
    }
}

#[derive(Deserialize)]
struct GQLTransaction {
    edges: Vec<GQLEdge>
}

#[derive(Deserialize)]
struct GQLEdge {
    node: GQLNode
}

#[derive(Deserialize)]
struct GQLNode {
    id: String
}

const IPFS_GQL_QUERY: &'static str = "{
    transactions(
      tags:[{name:\"IPFS-CID\",values:[\"{}\"]}],first:1,order:ASC
    ) {
      edges {
        cursor
        node {id}
      }
      pageInfo{hasNextPage}
    }
  }
";

pub async fn get_tx_data_ipfs(
    bundlers: Data<Vec<String>>,
    client: Data<ClientWithMiddleware>,
    path: Path<(String)>,
) -> actix_web::Result<HttpResponse> {
    let cid = path.into_inner();

    dbg!(&cid);

    let tx_id_get_futures = bundlers.iter().map(|b| {
        let cid: String = cid.clone();
        let client = client.clone();

        debug!("Checking CID {}", cid);

        let body = String::from(IPFS_GQL_QUERY).replace("{}", &cid);

        let url = format!("{}/graphql", b);
            // Create request builder, configure request and send
            async move {
                let res = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(json!({ "query": body }).to_string())
                    .timeout(Duration::from_millis(30 * 1000))
                    .send()
                    .await
                    .unwrap();

                    dbg!(res.status());

                    dbg!(json!({ "query": body }).to_string());
                    // dbg!(res.text().await.unwrap());
                let body = res
                    .json::<GQLResponse>()
                    .await
                    .unwrap();


                (b, url.clone(), body.data.transactions.edges)
            }
        });

    let tx_id_get_s = futures::stream::iter(tx_id_get_futures);

    let tx_id_get_executors = tx_id_get_s.buffer_unordered(2).skip_while(|(b, _, r)| {
        std::future::ready(r.first().is_none())
    }).next().await;



    let final_tx_id = match tx_id_get_executors {
        Some((_, _, cid)) => cid.first().unwrap().node.id.clone(),
        None => return Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
    };

    dbg!(&final_tx_id);


    let data_get_futures = bundlers.iter().map(|b| {
        let tx_id: String = final_tx_id.clone();
        let client = client.clone();

        debug!("Checking {} for tx id {}", b, tx_id);

        let url = format!("{}/tx/{}/data", b, tx_id);
            // Create request builder, configure request and send
            async move { (b, url.clone(), client.head(url).timeout(Duration::from_millis(30 * 1000)).send().await) }

        });

    let data_get_s = futures::stream::iter(data_get_futures);

    let data_get_executors = data_get_s.buffer_unordered(2).skip_while(|(b, _, r)| {
        match r {
                Ok(req) => {
                    if req.status().is_success() {
                        info!("Found {} at {}", final_tx_id, b);
                    debug!("Headers {:?}", req.headers());
                        std::future::ready(false)
                    } else {
                        debug!("Not success getting from {} - {}", b, req.status());

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    error!(
                        "Error occurred while getting {} from {} - {}",
                        final_tx_id, b, e
                    );
                    std::future::ready(true)
                }
            }
    }).next().await;

    match data_get_executors {
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

const MUTABLE_GQL_QUERY: &'static str = "{
    transactions(
      tags:[{name:\"Root-TX\",values:[\"{}\"]}],first:1,order:DESC
    ) {
      edges {
        cursor
        node {id}
      }
      pageInfo{hasNextPage}
    }
  }
";

pub async fn get_tx_data_mutable(
    bundlers: Data<Vec<String>>,
    client: Data<ClientWithMiddleware>,
    path: Path<(String)>,
) -> actix_web::Result<HttpResponse> {
    let root_id = path.into_inner();

    dbg!(&root_id);

    let tx_id_get_futures = bundlers.iter().map(|b| {
        let root_id: String = root_id.clone();
        let client = client.clone();

        debug!("Checking root tx {}", root_id);

        let body = String::from(MUTABLE_GQL_QUERY).replace("{}", &root_id);

        let url = format!("{}/graphql", b);
            // Create request builder, configure request and send
            async move {
                let res = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(json!({ "query": body }).to_string())
                    .timeout(Duration::from_millis(30 * 1000))
                    .send()
                    .await
                    .unwrap();

                    dbg!(res.status());

                    dbg!(json!({ "query": body }).to_string());
                    // dbg!(res.text().await.unwrap());
                let body = res
                    .json::<GQLResponse>()
                    .await
                    .unwrap();


                (b, url.clone(), body.data.transactions.edges)
            }
        });

    let tx_id_get_s = futures::stream::iter(tx_id_get_futures);

    let tx_id_get_executors = tx_id_get_s.buffer_unordered(2).skip_while(|(b, _, r)| {
        std::future::ready(r.first().is_none())
    }).next().await;



    let final_tx_id = match tx_id_get_executors {
        Some((_, _, root_id)) => root_id.first().unwrap().node.id.clone(),
        None => root_id,
    };

    dbg!(&final_tx_id);


    let data_get_futures = bundlers.iter().map(|b| {
        let tx_id: String = final_tx_id.clone();
        let client = client.clone();

        debug!("Checking {} for tx id {}", b, tx_id);

        let url = format!("{}/tx/{}/data", b, tx_id);
            // Create request builder, configure request and send
            async move { (b, url.clone(), client.head(url).timeout(Duration::from_millis(30 * 1000)).send().await) }

        });

    let data_get_s = futures::stream::iter(data_get_futures);

    let data_get_executors = data_get_s.buffer_unordered(2).skip_while(|(b, _, r)| {
        match r {
                Ok(req) => {
                    if req.status().is_success() {
                        info!("Found {} at {}", final_tx_id, b);
                    debug!("Headers {:?}", req.headers());
                        std::future::ready(false)
                    } else {
                        debug!("Not success getting from {} - {}", b, req.status());

                        std::future::ready(true)
                    }
                }
                Err(e) => {
                    error!(
                        "Error occurred while getting {} from {} - {}",
                        final_tx_id, b, e
                    );
                    std::future::ready(true)
                }
            }
    }).next().await;

    match data_get_executors {
        Some((_, url, r)) => {
            let r = r.unwrap();
            if let Some(h) = r.headers().get("Content-Length") {
                return Ok(HttpResponse::Found()
                    .insert_header(("Content-Length", h))
                    .insert_header(("Location", url))
                    .insert_header(("Cache-Control", "max-age=0"))
                    .finish());
            } else {
                return Ok(HttpResponse::Found()
                    .insert_header(("Location", url))
                    .insert_header(("Cache-Control", "max-age=0"))
                    .finish());
            }
        },
        None => Ok(HttpResponse::NotFound()
        .insert_header(("Cache-Control", "max-age=0"))
        .finish())
    }
}