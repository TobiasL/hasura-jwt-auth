use bcrypt::verify;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use tide::convert::json;
use tide::{Request, Response, Result};

use crate::jwt;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    password_hash: String,
    default_role: String,
}

const GET_USER_QUERY: &str = "SELECT * from users where email = $1";

pub async fn login(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let found_user: Option<UserRow> = sqlx::query_as(GET_USER_QUERY)
        .bind(credentials.email)
        .fetch_optional(&db)
        .await?;

    if let Some(user) = found_user {
        let valid = verify(credentials.password, &user.password_hash)?;

        if !valid {
            return Ok(Response::builder(401).body("Wrong password").build());
        }

        let user_session =
            jwt::session::create_session(&db, &jwt_secret, user.id, user.default_role).await?;

        Ok(Response::builder(200).body(json!(user_session)).build())
    } else {
        Ok(Response::builder(401).body("User not found").build())
    }
}
