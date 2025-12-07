use crate::routes::login::{login, sign_in};
use config::launch_service;

mod db;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    launch_service!(
        service: "auth",
        routes: [login, sign_in]
    );
    Ok(())
}
