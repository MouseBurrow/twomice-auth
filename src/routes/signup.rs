use crate::errors::AuthError;
use crate::password_utils::hash_password;
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignBody {
    pub username: String,
    pub password: String,
}

#[post("/signup")]
pub async fn signup(
    app: web::Data<AppData>,
    body: web::Json<SignBody>,
) -> Result<HttpResponse, AuthError> {
    let password_hash = hash_password(&body.password).map_err(|_| AuthError::PasswordHashFailed)?;

    let token: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT create_account($1, $2)",
        binds = [&body.username, password_hash]
    )?;

    Ok(HttpResponse::Ok()
        .cookie(SessionToken::create_cookie(
            app.config.app_env != AppEnvs::DEV,
            token,
        ))
        .json(serde_json::json!({ "ok": true })))
}
