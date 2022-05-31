use crate::db::refresh_tokens::create_refresh_token;
use crate::jwt::token::create_token;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tide::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    jwt_token: String,
    jwt_token_expires_minutes: u32,
    refresh: String,
    refresh_expires_days: u32,
}

pub async fn create_session(
    db: &PgPool,
    jwt_secret: &String,
    user_id: &Uuid,
    default_role: &String,
    org_id: &Option<Uuid>,
) -> Result<UserSession> {
    let access_token = create_token(&jwt_secret, *user_id, default_role.to_string(), *org_id)?;
    let refresh_token = create_refresh_token(&db, user_id).await?;

    Ok(UserSession {
        jwt_token: access_token,
        jwt_token_expires_minutes: 15,
        refresh: refresh_token.to_string(),
        refresh_expires_days: 60,
    })
}
