use crate::errors::AuthError;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use custom_headers::user_id::UserId;

pub async fn account(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<service::AccountInfo>, AuthError> {
    let info = service::get_account_info(&app.pool, user_id.into()).await?;

    Ok(Json(info))
}
