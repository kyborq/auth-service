use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}
