use crate::errors::AuthError;
use crate::password_utils::hash_password;
use axum::extract::State;
use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SignBody {
    pub username: String,
    pub password: String,
}

pub async fn signup(
    State(app): State<AppData>,
    Json(body): Json<SignBody>,
) -> Result<(HeaderMap, Json<serde_json::Value>), AuthError> {
    let password_hash =
        hash_password(&body.password).map_err(|_| AuthError::PasswordHashFailed)?;

    let token: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT create_account($1, $2)",
        binds = [&body.username, password_hash]
    )?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, token);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok((headers, Json(json!({ "ok": true }))))
}
