use crate::db::auth::AuthHandler;
use crate::db::errors::AuthError;
use crate::utils::password_utils::hash_password;
use actix_web::{post, web, HttpResponse, Responder};
use config::app_data::AppData;
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginBody {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(app: web::Data<AppData>, body: web::Json<LoginBody>) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    match AuthHandler::login_account(&app.pool, username, password).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "token": token
        })),
        Err(AuthError::InvalidPassword) => HttpResponse::Unauthorized().body("Invalid password"),
        Err(AuthError::UserNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(AuthError::Unexpected(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {:?}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/sign_in")]
pub async fn sign_in(app: web::Data<AppData>, body: web::Json<LoginBody>) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    let hashed = hash_password(password).unwrap();

    match AuthHandler::create_account(&app.pool, username, hashed.as_str()).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "token": token
        })),
        Err(AuthError::UsernameExists) => {
            HttpResponse::Unauthorized().body("Account already exists!")
        }
        Err(AuthError::Unexpected(err)) => {
            HttpResponse::InternalServerError().body(format!("DB error: {:?}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
