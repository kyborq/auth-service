use mongodb::{options::ClientOptions, Client, Database};

use crate::utils::get_env;

pub async fn init_db() -> Database {
    let mongo_uri = get_env("MONGODB_URI");
    let database_name = get_env("DB_NAME");

    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&database_name);

    db
}
