use crate::errors::AuthError;
use axum::extract::State;
use axum::Json;
use chrono::{DateTime, Utc};
use config::app_data::AppData;
use custom_headers::user_id::UserId;
use easy_db::db_call;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct AccountInfo {
    username: String,
    is_admin: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub async fn account(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<AccountInfo>, AuthError> {
    let account_info: AccountInfo = db_call!(
        pool = &app.pool,
        query = ONE ROW "SELECT * FROM get_account_info($1)",
        binds = [user_id]
    )?;

    Ok(Json(account_info))
}
