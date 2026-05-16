use crate::errors::AuthError;
use crate::password_utils::hash_password;
use crate::service;
use axum::extract::State;
use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
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
    let password_hash = hash_password(&body.password).map_err(|_| AuthError::PasswordHashFailed)?;

    let token = service::create_account(&app.pool, &body.username, &password_hash).await?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, token);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok((headers, Json(json!({ "ok": true }))))
}
