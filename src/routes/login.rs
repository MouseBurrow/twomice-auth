use crate::errors::AuthError;
use crate::password_utils::verify_password;
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
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(app): State<AppData>,
    Json(body): Json<LoginBody>,
) -> Result<(HeaderMap, Json<serde_json::Value>), AuthError> {
    let stored_hash = service::get_password_hash(&app.pool, &body.username).await?;

    if verify_password(&body.password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id =
        sqlx::query_scalar::<_, uuid::Uuid>("SELECT id FROM accounts WHERE username = $1")
            .bind(&body.username)
            .fetch_one(&app.pool)
            .await
            .map_err(easy_errors::map_sqlx_error::<AuthError>)?;

    let session_token = service::create_session(&app.pool, user_id).await?;

    let secure = app.config.app_env != AppEnvs::DEV;
    let cookie = SessionToken::cookie_value(secure, session_token);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok((headers, Json(json!({ "ok": true }))))
}
