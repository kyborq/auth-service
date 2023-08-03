use mongodb::{Collection, Database};

use crate::models::{Credentials, User};

pub fn db_login_user(db: &Database, credentials: Credentials) -> Option<User> {
    let users: Collection<User> = db.collection("users");

    None
}
