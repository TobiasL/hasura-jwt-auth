use crate::db::init::TableConn;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tide::{Error, Result};

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

pub async fn create_refresh_token(db: &PgPool, refresh_expires_in_days: &u64, user_id: &Uuid) -> Result<Uuid> {
    let add_refresh_token_query = format!(
        "INSERT INTO refresh_tokens (user_id, expires_at)
        VALUES ($1, current_timestamp + interval '{refresh_expires_in_days} days')
        RETURNING refresh_token;"
    );

    let token: RefreshToken = sqlx::query_as(&add_refresh_token_query).bind(user_id).fetch_one(db).await?;

    Ok(token.refresh_token)
}

const GET_REFRESH_TOKEN_QUERY: &str = "
  SELECT refresh_tokens.user_id, users.default_role, NULL AS org_id FROM refresh_tokens
  LEFT JOIN users ON users.id = refresh_tokens.user_id
  WHERE refresh_token = $1 AND expires_at > current_timestamp;
";

fn get_user_org_query(configured_table_conn: &Option<TableConn>) -> String {
    match configured_table_conn {
        None => GET_REFRESH_TOKEN_QUERY.to_string(),
        Some(TableConn {
            column_name,
            table_name,
        }) => {
            format!(
                "SELECT refresh_tokens.user_id, users.default_role, org_table.{column_name} AS org_id
                  FROM refresh_tokens LEFT JOIN users ON users.id = refresh_tokens.user_id
                  LEFT JOIN {table_name} AS org_table ON org_table.user_id = users.id
                  WHERE refresh_token = $1 AND expires_at > current_timestamp;"
            )
        }
    }
}

pub async fn get_refresh_token(
    db: &PgPool,
    configured_table_conn: &Option<TableConn>,
    refresh_token: &Uuid,
) -> Result<Option<RefreshUserRow>> {
    let user_query = get_user_org_query(&configured_table_conn);

    sqlx::query_as(&user_query)
        .bind(refresh_token)
        .fetch_optional(db)
        .await
        .map_err(|_err| Error::from_str(500, "Error getting refresh token"))
}
