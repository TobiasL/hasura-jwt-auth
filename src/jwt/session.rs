use crate::db::refresh_tokens::create_refresh_token;
use crate::jwt::token::create_token;
use actix_web::{error, Error};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug)]
pub struct UserSession {
    pub jwt_token: String,
    pub jwt_max_age_seconds: u64,
    pub refresh: String,
    pub refresh_max_age_seconds: u64,
}

pub async fn create_session(
    db: &PgPool,
    jwt_secret: &String,
    jwt_expires_in_minutes: &u64,
    refresh_expires_in_days: &u64,
    user_id: &Uuid,
    default_role: &String,
    org_id: &Option<Uuid>,
) -> Result<UserSession, Error> {
    let access_token = create_token(
        &jwt_secret,
        jwt_expires_in_minutes,
        *user_id,
        default_role.to_string(),
        *org_id,
    )
    .map_err(|_err| error::ErrorInternalServerError("Error creating token"))?;

    let refresh_token = create_refresh_token(&db, refresh_expires_in_days, user_id).await?;

    Ok(UserSession {
        jwt_token: access_token,
        jwt_max_age_seconds: jwt_expires_in_minutes * 60,
        refresh: refresh_token.to_string(),
        refresh_max_age_seconds: refresh_expires_in_days * (24 * 60 * 60),
    })
}
