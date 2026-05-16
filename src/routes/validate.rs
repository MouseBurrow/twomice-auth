use crate::errors::AuthError;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use custom_headers::session_token::SessionToken;
use serde_json::json;

pub async fn validate(
    State(app): State<AppData>,
    session_token: SessionToken,
) -> Result<Json<serde_json::Value>, AuthError> {
    let user_id = service::validate_token(&app.pool, &session_token.0).await?;

    Ok(Json(json!({ "user_id": user_id })))
}
