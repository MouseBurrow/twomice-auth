use crate::services::login::{login, sign_in};
use config::launch_service;

mod db;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    launch_service!(
        service: "AUTH",
        routes: [login, sign_in]
    );
    Ok(())
}
