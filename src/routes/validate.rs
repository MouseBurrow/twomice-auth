use crate::db::auth::AuthHandler;
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use custom_headers::session_token::SessionToken;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct ValidateResponse {
    user_id: Option<Uuid>,
}

#[post("/validate")]
pub async fn validate(app: web::Data<AppData>, session_token: SessionToken) -> HttpResponse {
    match AuthHandler::validate_token(&app.pool, session_token.into()).await {
        Ok(result) => HttpResponse::Ok().json(ValidateResponse { user_id: result }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
