use crate::utils::errors::AuthError;
use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Utc};
use config::app_data::AppData;
use custom_headers::user_id::UserId;
use easy_db::db_call;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
struct AccountInfo {
    username: String,
    is_admin: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[get("/account")]
pub async fn account(app: web::Data<AppData>, user_id: UserId) -> HttpResponse {
    let result: Result<AccountInfo, AuthError> = db_call!(
        pool = &app.pool,
        query = ONE ROW "SELECT * FROM get_account_info($1)",
        binds = [user_id]
    );

    match result {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(AuthError::AccountNotFound) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "account_not_found",
            "message": "An account with given ID does not exists."
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
