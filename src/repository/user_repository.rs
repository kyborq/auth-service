use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
    Collection, Database,
};

use crate::models::{Credentials, User};

pub async fn db_login_user(db: &Database, credentials: Credentials) -> Option<User> {
    let users: Collection<User> = db.collection("users");

    let filter: Document = doc! {
      "login": credentials.login,
      "password": credentials.password
    };
    let options = FindOneOptions::builder().build();

    let result = users.find_one(filter, options).await;

    match result {
        Ok(user) => user,
        Err(error) => {
            println!("👽 Failed to execute query... Idk why");
            println!("👽 Here is error: {:?}", error);
            None
        }
    }
}
