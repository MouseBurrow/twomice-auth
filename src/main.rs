use crate::routes::account::account;
use crate::routes::login::login;
use crate::routes::logout::logout;
use crate::routes::signup::signup;
use crate::routes::validate::validate;
use actix_web::{web, App, HttpServer};
use config::app_data::AppData;
use config::config::Config;
use config::logger;

pub(crate) mod errors;
pub(crate) mod password_utils;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let config = Config::load("auth");
    let app_data = AppData::new(config.clone()).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(validate)
            .service(login)
            .service(signup)
            .service(logout)
            .service(account)
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
