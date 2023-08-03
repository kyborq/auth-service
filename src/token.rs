use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: String,
    pub exp: usize,
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
