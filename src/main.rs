mod errors;
pub(crate) mod password_utils;
mod routes;

use crate::routes::account::account;
use crate::routes::login::login;
use crate::routes::logout::logout;
use crate::routes::signup::signup;
use crate::routes::validate::validate;
use axum::routing::{get, post};
use axum::Router;
use config::app_data::AppData;
use config::config::Config;
use config::logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let config = Config::load("auth");
    let app_data = AppData::new(config.clone()).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    let app = Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/logout", post(logout))
        .route("/validate", post(validate))
        .route("/account", get(account))
        .with_state(app_data);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
