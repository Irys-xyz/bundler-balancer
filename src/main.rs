mod routes;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{App, HttpServer, middleware::Logger, web::{self, Data}};
use routes::index::index;
// use sqlx::postgres::PgPoolOptions;

use crate::routes::{get_tx_data::get_tx_data, post_tx::post_tx};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();


    let bundlers_file = std::fs::read_to_string("bundlers.json").unwrap();

    let bundlers = serde_json::from_str::<Vec<String>>(bundlers_file.as_str()).unwrap()
        .into_iter()
        .map(|host| format!("http://{}", host))
        .collect::<Vec<_>>();

    info!("Starting up server...");

    let port = std::env::var("PORT").unwrap();
    info!("Running on port {}", port);

    HttpServer::new(move || {
        let client = awc::Client::new();

        App::new()
            .wrap(Logger::new("%r %s %a %Dms"))
            .app_data(Data::new(client))
            .app_data(Data::new(bundlers.clone()))
            .service(
                web::scope("")
                .route("/", web::get().to(index))
                .route("/info", web::get().to(index))
                .route("/tx/{tx_id}/data", web::get().to(get_tx_data))
                .route("/tx", web::post().to(post_tx))
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
