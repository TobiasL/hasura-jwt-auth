use crate::db::refresh_tokens::create_refresh_token;
use crate::jwt::token::create_token;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tide::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    jwt_token: String,
    jwt_token_expires_minutes: u64,
    refresh: String,
    refresh_expires_days: u64,
}

pub async fn create_session(
    db: &PgPool,
    jwt_secret: &String,
    jwt_expires_in_minutes: &u64,
    refresh_expires_in_days: &u64,
    user_id: &Uuid,
    default_role: &String,
    org_id: &Option<Uuid>,
) -> Result<UserSession> {
    let access_token = create_token(
        &jwt_secret,
        jwt_expires_in_minutes,
        *user_id,
        default_role.to_string(),
        *org_id,
    )?;
    let refresh_token = create_refresh_token(&db, refresh_expires_in_days, user_id).await?;

    Ok(UserSession {
        jwt_token: access_token,
        jwt_token_expires_minutes: *jwt_expires_in_minutes,
        refresh: refresh_token.to_string(),
        refresh_expires_days: *refresh_expires_in_days,
    })
}
