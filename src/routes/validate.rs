use crate::errors::AuthError;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use serde_json::json;
use uuid::Uuid;

pub async fn validate(
    State(app): State<AppData>,
    session_token: SessionToken,
) -> Result<Json<serde_json::Value>, AuthError> {
    let user_id: Option<Uuid> = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT validate_token($1)",
        binds = [session_token]
    )?;

    Ok(Json(json!({ "user_id": user_id })))
}
