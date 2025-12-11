use crate::routes::login::login;
use crate::routes::sign_in::sign_in;
use crate::routes::validate::validate;
use config::launch_service;

mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    launch_service!(
        service: "auth",
        routes: [validate, login, sign_in]
    );
    Ok(())
}
