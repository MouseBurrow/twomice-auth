use crate::errors::AuthError;
use crate::password_utils::hash_password;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
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

    let cookie = Cookie::build("session_token", token)
        .http_only(true)
        .secure(app.config.app_env != AppEnvs::DEV)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(30))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "ok": true })))
}
