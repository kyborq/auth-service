use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOneOptions,
    Collection, Database,
};

use crate::models::User;

pub async fn db_get_user_by_login(db: &Database, login: String) -> Option<User> {
    let users: Collection<User> = db.collection("users");

    let filter: Document = doc! {
      "login": login,
    };
    let options = FindOneOptions::builder().build();

    let result = users.find_one(filter, options).await;

    match result {
        Ok(user) => user,
        Err(error) => {
            println!("😿 db_get_user_by_login 🪲 {:?}", error);
            None
        }
    }
}

pub async fn db_get_user_by_id(db: &Database, id: String) -> Option<User> {
    let users: Collection<User> = db.collection("users");

    let filter: Document = doc! {
      "id": ObjectId::from_str(&id).unwrap(),
    };
    let options = FindOneOptions::builder().build();

    let result = users.find_one(filter, options).await;

    match result {
        Ok(user) => user,
        Err(error) => {
            println!("😿 db_get_user_by_id 🪲 {:?}", error);
            None
        }
    }
}
