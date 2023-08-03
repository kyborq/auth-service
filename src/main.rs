mod api;
mod models;
mod repository;
mod token;
mod utils;

use actix_web::{web, App, HttpServer};
use api::{login_user, register_user};
use dotenv::dotenv;
use utils::get_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_url = get_env("SERVER_URL");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/app")
                .route("/login", web::post().to(login_user))
                .route("/register", web::post().to(register_user)),
        )
    })
    .bind(server_url)?
    .run()
    .await
}
