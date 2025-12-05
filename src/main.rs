use crate::db::auth::AuthHandler;
use crate::db::errors::AuthError;
use crate::services::password_service::hash_password;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod db;
mod models;
mod services;
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// TEST
#[post("/sign_in")]
async fn sign_in(
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

#[post("/login")]
async fn login(
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    match AuthHandler::login_account(&pool, username, password).await {
        Ok((user_id, token)) => {
            println!("Login attempt successful!");
            println!("User ID : {}", user_id);
            println!("Token   : {}", token);

            HttpResponse::Ok().json(serde_json::json!({
                "user_id": user_id,
                "token": token
            }))
        }
        Err(AuthError::InvalidPassword) => {
            return HttpResponse::Unauthorized().body("Invalid password");
        }

        Err(AuthError::UserNotFound) => {
            return HttpResponse::NotFound().body("User not found");
        }

        Err(AuthError::Db(err)) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {:?}", err));
        }

        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

#[get("/profile")]
async fn profile(
    pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let token = match query.get("token") {
        Some(t) => t.clone(),
        None => return HttpResponse::Unauthorized().body("Missing token"),
    };

    match AuthHandler::login_with_token(&pool, &token).await {
        Ok(user_id) => HttpResponse::Ok().json(user_id),
        Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // If it is dev mode, load env variables from .env
    #[cfg(debug_assertions)]
    Config::load_local_env();

    // Connect to the database
    let config: Config = Config::init("AUTH");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await?;

    // Start listening endpoint
    let port = env::var("PORT")?;
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting Gateway at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(sign_in)
            .service(profile)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
