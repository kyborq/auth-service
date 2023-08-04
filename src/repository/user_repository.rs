use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneOptions, InsertOneOptions},
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
            println!("😿 db_login_user 🪲 {:?}", error);
            None
        }
    }
}

pub async fn db_register_user(db: &Database, credentials: Credentials) -> Option<ObjectId> {
    let users: Collection<User> = db.collection("users");

    let user = User {
        id: ObjectId::new(),
        login: credentials.login.clone(),
        password: credentials.password.clone(),
    };
    let options = InsertOneOptions::builder().build();

    let result = users.insert_one(user, options).await;

    match result {
        Ok(user) => user.inserted_id.as_object_id(),
        Err(error) => {
            println!("😿 db_register_user 🪲 {:?}", error);
            None
        }
    }
}
