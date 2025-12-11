use crate::utils::errors::AuthError;
use actix_web::{post, web, HttpResponse};
use burrow_db::db_call;
use config::app_data::AppData;
use custom_headers::session_token::SessionToken;
use uuid::Uuid;

#[post("/validate")]
pub async fn validate(app: web::Data<AppData>, session_token: SessionToken) -> HttpResponse {
    let result: Result<Option<Uuid>, AuthError> = db_call!(
        pool = &app.pool,
        query = sqlx::query_scalar(r#"SELECT validate_token($1)"#),
        binds = [session_token],
        error = AuthError
    );

    match result {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": result
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
