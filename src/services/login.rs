use crate::db::auth::AuthHandler;
use crate::db::errors::AuthError;
use crate::utils::password_utils::hash_password;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("login")]
pub async fn login(
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    match AuthHandler::login_account(&pool, username, password).await {
        Ok((user_id, token)) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": user_id,
            "token": token
        })),
        Err(AuthError::InvalidPassword) => HttpResponse::Unauthorized().body("Invalid password"),
        Err(AuthError::UserNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(AuthError::Db(err)) => {
            HttpResponse::InternalServerError().body(format!("Database error: {:?}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("sign_in")]
pub async fn sign_in(
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    let hashed = hash_password(password).unwrap();

    match AuthHandler::create_account(&pool, username, hashed.as_str()).await {
        Ok((user_id, token)) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": user_id,
            "token": token
        })),
        Err(AuthError::UsernameExists) => {
            HttpResponse::Unauthorized().body("Account already exists!")
        }
        Err(AuthError::Db(err)) => {
            HttpResponse::InternalServerError().body(format!("DB error: {:?}", err))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
