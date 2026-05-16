use crate::errors::AuthError;
use crate::service;
use axum::extract::State;
use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use serde_json::json;

pub async fn logout(
    State(app): State<AppData>,
    session_token: SessionToken,
) -> Result<(HeaderMap, Json<serde_json::Value>), AuthError> {
    service::logout_session(&app.pool, &session_token.0).await?;

    let cookie = SessionToken::clear_cookie_value(app.config.app_env != AppEnvs::DEV);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok((headers, Json(json!({"ok": true}))))
}
