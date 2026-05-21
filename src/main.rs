mod errors;
pub(crate) mod password_utils;
mod routes;
pub(crate) mod service;

use axum::routing::{get, post};
use axum::Router;
use config::server;
use routes::account::account;
use routes::login::login;
use routes::logout::logout;
use routes::signup::signup;
use routes::validate::validate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve("auth", Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/logout", post(logout))
        .route("/validate", post(validate))
        .route("/account", get(account))
    ).await
}
