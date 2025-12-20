use crate::utils::errors::AuthError;
use crate::utils::password_utils::verify_password;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use easy_db::db_call;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct LoginBody {
    pub username: String,
    pub password: String,
}

pub async fn login_account(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, AuthError> {
    let stored_hash: String = db_call!(
        pool = pool,
        query = ONE COLUMN "SELECT get_password_hash($1)",
        binds = [username],
    )?;

    if verify_password(password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id: Uuid = db_call!(
        pool = pool,
        query = ONE COLUMN "SELECT id FROM accounts WHERE username=$1",
        binds = [username]
    )?;

    let session_token: String = db_call!(
        pool = pool,
        query = ONE COLUMN "SELECT create_session($1)",
        binds = [user_id]
    )?;

    Ok(session_token)
}

#[post("/login")]
pub async fn login(app: web::Data<AppData>, body: web::Json<LoginBody>) -> HttpResponse {
    let username = &body.username;
    let password = &body.password;

    let app_env = &app.config.app_env;
    match login_account(&app.pool, username, password).await {
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
        Err(AuthError::InvalidPassword) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "invalid_password",
            "message": "Invalid password"
        })),
        Err(AuthError::UserNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "user_not_found",
            "message": "User not found"
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
