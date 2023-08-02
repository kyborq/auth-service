mod routes;
mod token;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use routes::{echo, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
