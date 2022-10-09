use crate::db::init::OrgTableInfo;
use actix_web::{error, Error};
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct RefreshUserRow {
    pub user_id: Uuid,
    pub default_role: String,
    pub org_id: Option<Uuid>,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct RefreshToken {
    refresh_token: Uuid,
}

pub async fn create_refresh_token(db: &PgPool, refresh_expires_in_days: &u64, user_id: &Uuid) -> Result<Uuid, Error> {
    let add_refresh_token_query = format!(
        "INSERT INTO refresh_tokens (user_id, expires_at)
        VALUES ($1, current_timestamp + interval '{refresh_expires_in_days} days')
        RETURNING refresh_token;"
    );

    let token: RefreshToken = sqlx::query_as(&add_refresh_token_query)
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(token.refresh_token)
}

const GET_REFRESH_TOKEN_QUERY: &str = "
  SELECT refresh_tokens.user_id, users.default_role, NULL AS org_id FROM refresh_tokens
  LEFT JOIN users ON users.id = refresh_tokens.user_id
  WHERE refresh_token = $1 AND expires_at > current_timestamp FOR UPDATE OF refresh_tokens;
";

fn get_user_org_query(configured_table_conn: &Option<OrgTableInfo>) -> String {
    match configured_table_conn {
        None => GET_REFRESH_TOKEN_QUERY.to_string(),
        Some(OrgTableInfo {
            column_name,
            table_name,
        }) => {
            format!(
                "SELECT refresh_tokens.user_id, users.default_role, org_table.{column_name} AS org_id
                  FROM refresh_tokens LEFT JOIN users ON users.id = refresh_tokens.user_id
                  LEFT JOIN {table_name} AS org_table ON org_table.user_id = users.id
                  WHERE refresh_token = $1 AND expires_at > current_timestamp FOR UPDATE OF refresh_tokens;"
            )
        }
    }
}

const DELETE_REFRESH_TOKEN_QUERY: &str = "
  DELETE FROM refresh_tokens WHERE refresh_token = $1 AND expires_at > current_timestamp;
";

pub async fn get_and_delete_refresh_token(
    db: &PgPool,
    configured_table_conn: &Option<OrgTableInfo>,
    refresh_token: &Uuid,
) -> Result<Option<RefreshUserRow>, Error> {
    let user_query = get_user_org_query(&configured_table_conn);

    let mut tx = db
        .begin()
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    let user_row = sqlx::query_as(&user_query)
        .bind(refresh_token)
        .fetch_optional(&mut tx)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error getting refresh token"))?;

    sqlx::query(DELETE_REFRESH_TOKEN_QUERY)
        .bind(refresh_token)
        .execute(&mut tx)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error deleting refresh token"))?;

    tx.commit()
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(user_row)
}
