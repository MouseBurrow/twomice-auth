use crate::utils::errors::AuthError;
use crate::utils::password_utils::hash_password;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse, Responder};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use easy_db::db_call;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignBody {
    pub username: String,
    pub password: String,
}

#[post("/sign_in")]
pub async fn sign_in(app: web::Data<AppData>, body: web::Json<SignBody>) -> impl Responder {
    let username = &body.username;
    let password = &body.password;

    let password_hash = match hash_password(password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result: Result<String, AuthError> = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT create_account($1, $2)",
        binds = [username, password_hash]
    );

    let app_env = &app.config.app_env;
    match result {
        Ok(token) => {
            let cookie = Cookie::build("session_token", token)
                .http_only(true)
                .secure(*app_env != AppEnvs::DEV)
                .same_site(SameSite::Lax)
                .path("/")
                .finish();

            HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
                "ok": true
            }))
        }
        Err(AuthError::UsernameExists) => HttpResponse::Conflict().json(serde_json::json!({
            "error": "username_exists",
            "message": "Account already exists"
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
