mod routes;

use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();


    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/")
        )
    })
    .bind("127.0.0.1:8080")
    .run()
    .await;
}
