use actix_web::{
    cookie::{Cookie, SameSite},
    web::{Data, Json},
    HttpResponse,
};
use mongodb::{bson::oid::ObjectId, Database};

use crate::{
    models::{Credentials, User},
    repository::user_repository::db_login_user,
    services::token_service::generate_tokens,
};

pub async fn login_user(db: Data<Database>, credentials: Json<Credentials>) -> HttpResponse {
    let user_credentials = Credentials {
        login: credentials.login.clone(),
        password: credentials.password.clone(),
    };
    let result = db_login_user(db.as_ref(), user_credentials).await;

    match result {
        Some(user) => {
            let id = user.id.to_string();

            let tokens = generate_tokens(id);

            let cookie = Cookie::build("token", tokens.refresh_token)
                .same_site(SameSite::None)
                .path("/")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(format!("{{ 'access': '{}' }}", tokens.access_token))
        }
        None => {
            // Handle error there
            // Possible states: User is not found, password is not correct and other errors
            HttpResponse::Forbidden().into()
        }
    }
}

pub async fn register_user(credentials: Json<Credentials>) -> HttpResponse {
    let new_user = User {
        id: ObjectId::new(),
        login: credentials.login.clone(),
        password: credentials.password.clone(),
    };
    let not_found = true;

    if not_found {
        return HttpResponse::Forbidden().json(new_user).into();
    }

    HttpResponse::Ok().json(new_user)
}
