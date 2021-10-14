mod routes;

use actix_web::{App, HttpServer, web};
use routes::index::index;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();



    HttpServer::new(|| {
        App::new()
            .app_data()
            .service(
                web::scope("")
                .route("/", web::get().to(index))
                .route("/{tx_id}", web::get().to(index))
                .route("/tx", web::post().to(index))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
