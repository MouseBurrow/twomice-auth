use crate::errors::AuthError;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use config::app_data::AppData;
use config::app_envs::AppEnvs;
use custom_headers::session_token::SessionToken;
use easy_db::db_call;

#[post("/logout")]
pub async fn logout(
    app: web::Data<AppData>,
    session_token: SessionToken,
) -> Result<HttpResponse, AuthError> {
    let _: () = db_call!(
        pool = &app.pool,
        query = ONE COLUMN "SELECT logout_session($1)",
        binds = [session_token]
    )?;

    let cookie = Cookie::build("session_token", "")
        .http_only(true)
        .secure(app.config.app_env != AppEnvs::DEV)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"ok": true})))
}
