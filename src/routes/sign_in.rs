use crate::utils::errors::AuthError;
use crate::utils::password_utils::hash_password;
use actix_web::{post, web, HttpResponse, Responder};
use burrow_db::db_call;
use config::app_data::AppData;
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

    let password_hash = hash_password(password).unwrap();

    let result: Result<String, AuthError> = db_call!(
        pool = &app.pool,
        query = sqlx::query_scalar(r#"SELECT session_token FROM create_account($1, $2)"#),
        binds = [username, password_hash],
        error = AuthError
    );

    match result {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "session_token": token
        })),
        Err(AuthError::UsernameExists) => {
            HttpResponse::Unauthorized().body("Account already exists!")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
