use super::common::CredentialsBody;
use crate::errors::AuthError;
use crate::password_utils::verify_password;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use serde_json::json;

pub async fn login(
    State(app): State<AppData>,
    Json(body): Json<CredentialsBody>,
) -> Result<(axum::http::HeaderMap, Json<serde_json::Value>), AuthError> {
    let stored_hash = service::get_password_hash(&app.pool, &body.username).await?;

    if verify_password(&body.password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id = service::get_user_id_by_username(&app.pool, &body.username).await?;
    let session_token = service::create_session(&app.pool, user_id).await?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, session_token);

    Ok(SessionToken::set_cookie_response(cookie, json!({ "ok": true })))
}
