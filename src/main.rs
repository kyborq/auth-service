mod api;
mod database;
mod models;
mod repository;
mod services;
mod utils;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use api::{check_user_auth, login_user, register_user};
use database::init_db;
use dotenv::dotenv;
use utils::get_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_url = get_env("SERVER_URL");
    let db = init_db().await;
    let app_data = Data::new(db);

    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).service(
            web::scope("/api")
                .route("/login", web::post().to(login_user))
                .route("/register", web::post().to(register_user))
                .route("/check", web::get().to(check_user_auth)),
        )
    })
    .bind(server_url)?
    .run()
    .await
}
