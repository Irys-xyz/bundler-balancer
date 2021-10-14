use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .
    println!("Hello, world!");
}
