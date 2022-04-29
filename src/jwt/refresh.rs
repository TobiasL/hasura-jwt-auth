use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

use tide::{Error, Result};

const ADD_REFRESH_TOKEN_QUERY: &str = "
  INSERT INTO refresh_tokens (user_id, expires_at)
  VALUES ($1, current_timestamp + interval '60 minute')
  RETURNING refresh_token";

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct RefreshToken {
    refresh_token: Uuid,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
pub struct UserToken {
    pub user_id: Uuid,
    pub default_role: String,
}

const GET_REFRESH_TOKEN_QUERY: &str = "
  SELECT refresh_tokens.user_id, users.default_role FROM refresh_tokens
  LEFT JOIN users ON users.id = refresh_tokens.user_id
  WHERE refresh_token = $1 AND expires_at > current_timestamp;";

pub async fn create_refresh_token(db: &PgPool, user_id: Uuid) -> Result<Uuid> {
    let token: RefreshToken = sqlx::query_as(ADD_REFRESH_TOKEN_QUERY)
        .bind(user_id)
        .fetch_one(db)
        .await?;

    Ok(token.refresh_token)
}

pub async fn get_refresh_token(db: &PgPool, refresh_token: Uuid) -> Result<UserToken> {
    let user: Option<UserToken> = sqlx::query_as(GET_REFRESH_TOKEN_QUERY)
        .bind(refresh_token)
        .fetch_optional(db)
        .await?;

    match user {
        Some(user_found) => Ok(user_found),
        None => Err(Error::from_str(401, "Refresh token not found")),
    }
}
