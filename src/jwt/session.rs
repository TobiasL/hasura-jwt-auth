use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::jwt;
use tide::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    jwt_token: String,
    refresh: String,
}

pub async fn create_session(
    db: &PgPool,
    jwt_secret: &String,
    user_id: Uuid,
    default_role: String,
) -> Result<UserSession> {
    let access_token = jwt::token::create_token(&jwt_secret, user_id, default_role)?;
    let refresh_token = jwt::refresh::create_refresh_token(&db, user_id).await?;

    Ok(UserSession {
        jwt_token: access_token,
        refresh: refresh_token.to_string(),
    })
}
