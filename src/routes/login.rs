use crate::errors::AuthError;
use crate::password_utils::verify_password;
use axum::extract::State;
use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
use axum::Json;
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(app): State<AppData>,
    Json(body): Json<LoginBody>,
) -> Result<(HeaderMap, Json<serde_json::Value>), AuthError> {
    let stored_hash: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT get_password_hash($1)",
        binds = [&body.username],
    )?;

    if verify_password(&body.password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id: Uuid = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT id FROM accounts WHERE username = $1",
        binds = [&body.username]
    )?;

    let session_token: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT create_session($1)",
        binds = [user_id]
    )?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, session_token);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok((headers, Json(json!({ "ok": true }))))
}
