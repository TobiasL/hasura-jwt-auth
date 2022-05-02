use bcrypt::verify;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use tide::convert::json;
use tide::{Request, Response, Result};

use crate::jwt::session::create_session;
use crate::jwt::session::UserToken;
use crate::state::State;
use crate::user_org::TableConn;

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    password_hash: String,
    default_role: String,
    org_id: Option<Uuid>,
}

fn get_user_org_query(configured_table_conn: &Option<TableConn>) -> String {
    match configured_table_conn {
        None => "SELECT *, NULL AS org_id FROM users WHERE email = $1;".to_string(),
        Some(table_conn) => {
            format!(
                "SELECT *, org_table.{} AS org_id FROM users
                 LEFT JOIN {} AS org_table ON org_table.user_id = users.id
                 WHERE email = $1;",
                table_conn.column_name, table_conn.table_name
            )
        }
    }
}

pub async fn login(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let table_conn = req.state().table_conn.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let user_query = get_user_org_query(&table_conn);
    let found_user: Option<UserRow> = sqlx::query_as(&user_query)
        .bind(credentials.email)
        .fetch_optional(&db)
        .await?;

    if let Some(user) = found_user {
        let valid = verify(credentials.password, &user.password_hash)?;

        if !valid {
            return Ok(Response::builder(401).body("Wrong password").build());
        }

        let user_token = UserToken {
            user_id: user.id,
            default_role: user.default_role,
            org_id: user.org_id,
        };

        let user_session = create_session(&db, &jwt_secret, user_token).await?;

        Ok(Response::builder(200).body(json!(user_session)).build())
    } else {
        Ok(Response::builder(401).body("User not found").build())
    }
}
