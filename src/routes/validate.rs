use crate::errors::AuthError;
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use uuid::Uuid;

#[post("/validate")]
pub async fn validate(
    app: web::Data<AppData>,
    session_token: SessionToken,
) -> Result<HttpResponse, AuthError> {
    let user_id: Option<Uuid> = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT validate_token($1)",
        binds = [session_token]
    )?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": user_id
    })))
}
