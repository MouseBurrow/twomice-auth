use crate::db::auth::AuthHandler;
use actix_web::{post, web, HttpRequest, HttpResponse};
use config::app_data::AppData;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct ValidateResponse {
    user_id: Option<Uuid>,
}

#[post("/validate")]
pub async fn validate(app: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let token = match req.headers().get("X-Session-Token") {
        Some(val) => match val.to_str() {
            Ok(s) => s,
            Err(_) => {
                return HttpResponse::BadRequest().finish();
            }
        },
        None => {
            return HttpResponse::Ok().json(ValidateResponse { user_id: None });
        }
    };

    match AuthHandler::validate_token(&app.pool, token).await {
        Ok(result) => HttpResponse::Ok().json(ValidateResponse { user_id: result }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
