use crate::utils::errors::AuthError;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;

#[post("/logout")]
pub async fn logout(app: web::Data<AppData>, session_token: SessionToken) -> HttpResponse {
    let result: Result<(), AuthError> = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT logout_session($1)",
        binds = [session_token]
    );

    let app_env = &app.config.app_env;
    match result {
        Ok(_) => {
            let cookie = Cookie::build("session_token", "")
                .http_only(true)
                .secure(*app_env != AppEnvs::DEV)
                .same_site(SameSite::Lax)
                .path("/")
                .max_age(time::Duration::seconds(0))
                .finish();

            HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
                "ok": true
            }))
        }
        Err(AuthError::SessionNotFound) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "invalid_session",
            "message": "Session is invalid or expired"
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
