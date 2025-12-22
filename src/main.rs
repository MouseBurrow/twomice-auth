use crate::routes::account::account;
use crate::routes::login::login;
use crate::routes::logout::logout;
use crate::routes::signup::signup;
use crate::routes::validate::validate;
use config::launch_service;

mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    launch_service!(
        service: "auth",
        routes: [validate, login, signup, logout, account]
    );
    Ok(())
}
