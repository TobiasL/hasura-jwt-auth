use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tide::Result;

use crate::jwt::refresh::create_refresh_token;
use crate::jwt::token::create_token;

#[derive(Serialize, sqlx::FromRow)]
pub struct UserToken {
    pub user_id: Uuid,
    pub default_role: String,
    pub org_id: Option<Uuid>,
}

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
    user: UserToken,
) -> Result<UserSession> {
    let access_token = create_token(&jwt_secret, user.user_id, user.default_role, user.org_id)?;
    let refresh_token = create_refresh_token(&db, user.user_id).await?;

    Ok(UserSession {
        jwt_token: access_token,
        jwt_token_expires_minutes: 15,
        refresh: refresh_token.to_string(),
        refresh_expires_days: 60,
    })
}
