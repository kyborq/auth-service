use actix_web::{web, HttpResponse};

use crate::models::{Credentials, User};

pub async fn login_user(credentials: web::Json<Credentials>) -> HttpResponse {
    HttpResponse::Ok().json(credentials)
}

pub async fn register_user() -> HttpResponse {
    let new_user = User {
        login: "123".to_string(),
        password: "123".to_string(),
    };
    let not_found = true;

    if not_found {
        return HttpResponse::Forbidden().json(new_user).into();
    }

    HttpResponse::Ok().json(new_user)
}
