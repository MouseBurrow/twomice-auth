use crate::db::auth::AuthHandler;
use crate::db::errors::AuthError;
use crate::services::password_service::hash_password;
use actix_web::{post, web, web::Json, App, HttpResponse, HttpServer, Responder};
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
            println!("User not found → creating new account");

            let hashed = hash_password(password).unwrap();

            match AuthHandler::create_account(&pool, username, hashed.as_str()).await {
                Ok((user_id, token)) => {
                    return HttpResponse::Ok().json(serde_json::json!({
                        "user_id": user_id,
                        "token": token
                    }));
                }
                Err(AuthError::UsernameExists) => {
                    return HttpResponse::Unauthorized().body("Account already exists!");
                }
                Err(AuthError::Db(err)) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("DB error: {:?}", err));
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }

        Err(AuthError::Db(err)) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {:?}", err));
        }

        Err(_) => return HttpResponse::InternalServerError().finish(),
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
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
