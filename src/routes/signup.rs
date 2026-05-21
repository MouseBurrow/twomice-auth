use super::common::CredentialsBody;
use crate::errors::AuthError;
use crate::password_utils::hash_password;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use serde_json::json;

pub async fn signup(
    State(app): State<AppData>,
    Json(body): Json<CredentialsBody>,
) -> Result<(axum::http::HeaderMap, Json<serde_json::Value>), AuthError> {
    let password_hash = hash_password(&body.password).map_err(|_| AuthError::PasswordHashFailed)?;

    let token = service::create_account(&app.pool, &body.username, &password_hash).await?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, token);

    Ok(SessionToken::set_cookie_response(cookie, json!({ "ok": true })))
}
