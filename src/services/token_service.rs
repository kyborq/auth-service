use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::get_env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: String,
    pub exp: usize,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_token(user: String, secret: String, expiration_date: DateTime<Utc>) -> String {
    let claims: Claims = Claims {
        exp: expiration_date.timestamp() as usize,
        user,
    };

    let header: Header = Header::new(Algorithm::HS256);
    let encoding_key: EncodingKey = EncodingKey::from_secret(secret.as_ref());

    encode(&header, &claims, &encoding_key).unwrap()
}

pub fn generate_tokens(user: String) -> TokenPair {
    // To Local Storage on Client
    let access_secret = get_env("JWT_ACCESS_SECRET");
    let access_expiration = Utc::now() + Duration::minutes(30);
    let access_token = generate_token(user.clone(), access_secret, access_expiration);

    //  To HTTP Only Cookie
    let refresh_secret = get_env("JWT_REFRESH_SECRET");
    let refresh_expiration = Utc::now() + Duration::days(7);
    let refresh_token: String = generate_token(user.clone(), refresh_secret, refresh_expiration);

    TokenPair {
        access_token,
        refresh_token,
    }
}

#[allow(unused)]
pub fn validate_token(token: String, secret: String) -> Option<Claims> {
    let decoding_key: DecodingKey = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    let result = decode::<Claims>(token.as_ref(), &decoding_key, &validation);

    match result {
        Ok(token) => Some(token.claims),
        Err(_) => None,
    }
}
