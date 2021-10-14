mod routes;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{App, HttpServer, middleware::Logger, web};
use routes::index::index;
use sqlx::postgres::PgPoolOptions;

use crate::routes::{get_tx_data::get_tx_data, post_tx::post_tx};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();


    let database_url = std::env::var("DATABASE_URL").unwrap();


    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let bundlers_env = std::env::var("BUNDLERS").unwrap();
    let bundlers = serde_json::from_str::<Vec<String>>(bundlers_env.as_str()).unwrap()
        .into_iter()
        .map(|host| format!("http://{}", host))
        .collect::<Vec<_>>();


    HttpServer::new(move || {
        let client = awc::Client::new();

        App::new()
            .wrap(Logger::new("%a %r %U %Dms"))
            .app_data(client)
            .app_data(bundlers.clone())
            .service(
                web::scope("")
                .route("/", web::get().to(index))
                .route("/{tx_id}", web::get().to(get_tx_data))
                .route("/tx", web::post().to(post_tx))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
