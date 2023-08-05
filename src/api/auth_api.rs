use actix_web::{
    cookie::{Cookie, SameSite},
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use mongodb::Database;
use serde::Serialize;

use crate::{
    models::{Credentials, User},
    repository::{
        db_get_user_by_login, db_login_user, db_register_user, user_repository::db_get_user_by_id,
    },
    services::{token_service::generate_tokens, validate_token},
    utils::get_env,
};

#[derive(Serialize)]
pub struct LoginResult {
    pub user: User,
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorResult {
    pub code: String,
    pub message: String,
}

pub async fn login_user(db: Data<Database>, credentials: Json<Credentials>) -> HttpResponse {
    let credentials = credentials.into_inner();

    let result = db_login_user(db.as_ref(), credentials).await;

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

            let result = LoginResult {
                token: tokens.access_token,
                user,
            };

            HttpResponse::Ok().cookie(cookie).json(result)
        }
        None => {
            let error = ErrorResult {
                code: "USER_NOT_FOUND".to_string(),
                message: "User is not exist, check login and try again".to_string(),
            };
            HttpResponse::Forbidden().json(error).into()
        }
    }
}

pub async fn register_user(db: Data<Database>, credentials: Json<Credentials>) -> HttpResponse {
    let credentials = credentials.into_inner();

    let existense = db_get_user_by_login(db.as_ref(), credentials.login.clone()).await;

    if Option::is_some(&existense) {
        let result = db_register_user(db.as_ref(), credentials).await;

        match result {
            Some(id) => HttpResponse::Ok().json(id),
            None => {
                let error = ErrorResult {
                    code: "USER_NOT_ADDED".to_string(),
                    message: "User not added to database idk why".to_string(),
                };
                HttpResponse::Forbidden().json(error).into()
            }
        }
    } else {
        let error = ErrorResult {
            code: "USER_ALREADY_EXISTS".to_string(),
            message: "User already exists".to_string(),
        };
        HttpResponse::Forbidden().json(error).into()
    }
}

pub async fn check_user_auth(db: Data<Database>, req: HttpRequest) -> HttpResponse {
    let token_cookie = req.cookie("token");

    match token_cookie {
        Some(token) => {
            let token_string = token.value();

            // Validate token
            let access_secret = get_env("JWT_REFRESH_SECRET");
            let token_claims = validate_token(token_string.to_string(), access_secret);
            match token_claims {
                Some(claims) => {
                    // Check user from database
                    let user = db_get_user_by_id(db.as_ref(), claims.user).await;
                    if Option::is_some(&user) {
                        // Found, maybe check for blocked state or smt...
                        // But in this time we only return user
                        return HttpResponse::Ok().json(user);
                    } else {
                        // Not found probably deleted
                        let error = ErrorResult {
                            code: "USER_DELETED".to_string(),
                            message: "User was deleted".to_string(),
                        };
                        return HttpResponse::Forbidden().json(error).into();
                    }
                }
                None => {
                    // Not valid token
                    let error = ErrorResult {
                        code: "AUTH_EXPIRED".to_string(),
                        message: "User session expired, try login again".to_string(),
                    };
                    return HttpResponse::Forbidden().json(error).into();
                }
            }
        }
        None => {
            // Unauthorized (no token)
            let error = ErrorResult {
                code: "NOT_LOGGED_IN".to_string(),
                message: "You are not logged in".to_string(),
            };
            return HttpResponse::Forbidden().json(error).into();
        }
    }
}
