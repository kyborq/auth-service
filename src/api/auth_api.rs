use actix_web::{
    cookie::{Cookie, SameSite},
    web::Json,
    HttpResponse,
};
use chrono::{Duration, Utc};

use crate::{
    models::{Credentials, User},
    repository::user_repository::db_login_user,
    token::generate_token,
    utils::get_env,
};

pub async fn login_user(credentials: Json<Credentials>) -> HttpResponse {
    // db_login_user(db, credentials)

    // Replace for user id from database
    let test_id = "123456".to_string();

    // To Local Storage on Client
    let access_secret = get_env("JWT_ACCESS_SECRET");
    let access_expiration = Utc::now() + Duration::minutes(30);
    let access_token = generate_token(test_id.clone(), access_secret, access_expiration);

    //  To HTTP Only Cookie
    let refresh_secret = get_env("JWT_REFRESH_SECRET");
    let refresh_expiration = Utc::now() + Duration::days(7);
    let refresh_token: String = generate_token(test_id.clone(), refresh_secret, refresh_expiration);

    let cookie = Cookie::build("token", refresh_token)
        .same_site(SameSite::None)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(format!("{{ 'access': '{}' }}", access_token))
}

pub async fn register_user(credentials: Json<Credentials>) -> HttpResponse {
    let new_user = User {
        login: credentials.login.clone(),
        password: credentials.password.clone(),
    };
    let not_found = true;

    if not_found {
        return HttpResponse::Forbidden().json(new_user).into();
    }

    HttpResponse::Ok().json(new_user)
}
