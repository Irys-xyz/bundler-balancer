mod routes;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use routes::index::index;
use routes::sign_mock::sign_mock;
// use sqlx::postgres::PgPoolOptions;

use crate::routes::{
    get_tx_data::{get_tx_data, get_tx_meta, get_tx_data_manifest},
    post_tx::post_tx,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("h2", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply()
        .unwrap();

    let bundlers_file = std::fs::read_to_string("bundlers.json").unwrap();

    let bundlers = serde_json::from_str::<Vec<String>>(bundlers_file.as_str())
        .unwrap()
        .into_iter()
        .map(|host| format!("https://{}", host))
        .collect::<Vec<_>>();

    info!("Starting up server...");

    let port = std::env::var("PORT").unwrap();
    info!("Running on port {}", port);

    HttpServer::new(move || {
        let client = awc::Client::builder().disable_redirects().finish();

        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::new("%r %s %a %Dms"))
            .wrap(cors)
            .app_data(Data::new(client))
            .app_data(Data::new(bundlers.clone()))
            .service(
                web::scope("")
                    .route("/", web::get().to(index))
                    .route("/cosigner/sign", web::post().to(sign_mock))
                    .route("/info", web::get().to(index))
                    .route("/tx/{tx_id}/{field}", web::get().to(get_tx_data))
                    .route("/tx/{tx_id}/{field}", web::head().to(get_tx_data))
                    .route("/tx/{tx_id}", web::get().to(get_tx_meta))
                    .route("/tx", web::post().to(post_tx))
                    .route("/{tx_id:[a-zA-Z0-9_-]{43}}", web::get().to(get_tx_data))
                    .route("/{tx_id:[a-zA-Z0-9_-]{43}}/{path:.*}", web::get().to(get_tx_data_manifest)),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
