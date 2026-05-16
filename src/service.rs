use crate::errors::AuthError;
use chrono::{DateTime, Utc};
use easy_errors::map_sqlx_error;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct AccountInfo {
    pub username: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_password_hash(pool: &Pool<Postgres>, username: &str) -> Result<String, AuthError> {
    let hash: Option<String> =
        sqlx::query_scalar("SELECT password_hash FROM accounts WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx_error)?;

    hash.ok_or(AuthError::UserNotFound)
}

pub async fn create_account(
    pool: &Pool<Postgres>,
    username: &str,
    password_hash: &str,
) -> Result<String, AuthError> {
    let account_id: Uuid = sqlx::query_scalar(
        "INSERT INTO accounts (username, password_hash) VALUES ($1, $2) RETURNING id",
    )
    .bind(username)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx_error::<AuthError>)?;

    let token = generate_session_token();

    sqlx::query("INSERT INTO sessions (account_id, session_token) VALUES ($1, $2)")
        .bind(account_id)
        .bind(&token)
        .execute(pool)
        .await
        .map_err(map_sqlx_error::<AuthError>)?;

    Ok(token)
}

pub async fn create_session(pool: &Pool<Postgres>, account_id: Uuid) -> Result<String, AuthError> {
    let token = generate_session_token();

    sqlx::query("INSERT INTO sessions (account_id, session_token) VALUES ($1, $2)")
        .bind(account_id)
        .bind(&token)
        .execute(pool)
        .await
        .map_err(map_sqlx_error::<AuthError>)?;

    Ok(token)
}

pub async fn logout_session(pool: &Pool<Postgres>, session_token: &str) -> Result<(), AuthError> {
    let result = sqlx::query("DELETE FROM sessions WHERE session_token = $1")
        .bind(session_token)
        .execute(pool)
        .await
        .map_err(map_sqlx_error::<AuthError>)?;

    if result.rows_affected() == 0 {
        return Err(AuthError::SessionNotFound);
    }

    Ok(())
}

pub async fn validate_token(pool: &Pool<Postgres>, token: &str) -> Result<Option<Uuid>, AuthError> {
    let user_id: Option<Uuid> = sqlx::query_scalar(
        "UPDATE sessions SET last_used_at = NOW(), expires_at = (NOW() + INTERVAL '30 days')
         WHERE session_token = $1 AND expires_at > NOW()
         RETURNING account_id",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error::<AuthError>)?;

    Ok(user_id)
}

pub async fn get_account_info(
    pool: &Pool<Postgres>,
    user_id: Uuid,
) -> Result<AccountInfo, AuthError> {
    let info: Option<AccountInfo> = sqlx::query_as(
        "SELECT username, is_admin, created_at, updated_at FROM accounts WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error::<AuthError>)?;

    info.ok_or(AuthError::AccountNotFound)
}

fn generate_session_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
