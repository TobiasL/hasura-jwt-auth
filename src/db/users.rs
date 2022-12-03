use crate::db::init::OrgTableInfo;
use actix_web::{error, Error};
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub password_hash: String,
    pub default_role: String,
    pub org_id: Option<Uuid>,
}

const GET_USER_QUERY: &str = "SELECT *, NULL AS org_id FROM users WHERE email = $1;";

fn get_user_org_query(configured_table_conn: &Option<OrgTableInfo>) -> String {
    match configured_table_conn {
        None => GET_USER_QUERY.to_string(),
        Some(OrgTableInfo {
            column_name,
            table_name,
        }) => {
            format!(
                "SELECT *, org_table.{column_name} AS org_id FROM users
                 LEFT JOIN {table_name} AS org_table ON org_table.user_id = users.id
                 WHERE email = $1;"
            )
        }
    }
}

pub async fn get_user(
    db: &PgPool,
    table_conn: &Option<OrgTableInfo>,
    email: &String,
) -> Result<Option<UserRow>, Error> {
    let user_query = get_user_org_query(table_conn);

    sqlx::query_as(&user_query)
        .bind(email)
        .fetch_optional(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error getting user"))
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
pub struct LoggedInUserRow {
    pub id: Uuid,
    pub default_role: String,
}

const ADD_USER_QUERY: &str = "
  INSERT INTO users (email, password_hash, name)
  VALUES ($1, $2, $3)
  RETURNING id, default_role;
";

pub async fn create_user(
    db: &PgPool,
    email: &String,
    hashed_password: &String,
    name: &String,
) -> Result<LoggedInUserRow, Error> {
    let user = sqlx::query_as(ADD_USER_QUERY)
        .bind(email)
        .bind(hashed_password)
        .bind(name)
        .fetch_one(db)
        .await
        .map_err(|_err| error::ErrorBadRequest("User with email already exists"))?;

    Ok(user)
}
