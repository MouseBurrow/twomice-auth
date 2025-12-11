use crate::utils::errors::AuthError;
use crate::utils::password_utils::verify_password;
use actix_web::{post, web, HttpResponse};
use burrow_db::db_call;
use config::app_data::AppData;
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
        query = sqlx::query_scalar(r#"SELECT get_password_hash($1)"#),
        binds = [username],
        error = AuthError
    )?;

    if verify_password(password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id: Uuid = db_call!(
        pool = pool,
        query = sqlx::query_scalar(r#"SELECT id FROM accounts WHERE username=$1"#),
        binds = [username],
        error = AuthError
    )?;

    let session_token: String = db_call!(
        pool = pool,
        query = sqlx::query_scalar(r#"SELECT create_session($1)"#),
        binds = [user_id],
        error = AuthError
    )?;

    Ok(session_token)
}

#[post("/login")]
pub async fn login(app: web::Data<AppData>, body: web::Json<LoginBody>) -> HttpResponse {
    let username = &body.username;
    let password = &body.password;

    match login_account(&app.pool, username, password).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "session_token": token
        })),
        Err(AuthError::InvalidPassword) => HttpResponse::Unauthorized().body("Invalid password"),
        Err(AuthError::UserNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
