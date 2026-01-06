use crate::errors::AuthError;
use crate::password_utils::verify_password;
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct LoginBody {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    app: web::Data<AppData>,
    body: web::Json<LoginBody>,
) -> Result<HttpResponse, AuthError> {
    let stored_hash: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT get_password_hash($1)",
        binds = [&body.username],
    )?;

    if verify_password(&body.password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id: Uuid = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT id FROM accounts WHERE username = $1",
        binds = [&body.username]
    )?;

    let session_token: String = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT create_session($1)",
        binds = [user_id]
    )?;

    Ok(HttpResponse::Ok()
        .cookie(SessionToken::create_cookie(
            app.config.app_env != AppEnvs::DEV,
            session_token,
        ))
        .json(serde_json::json!({ "ok": true })))
}
