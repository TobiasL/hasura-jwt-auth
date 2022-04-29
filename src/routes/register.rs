use bcrypt::hash;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use tide::convert::json;
use tide::{Request, Response, Result};

use crate::jwt;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct LoggedInUser {
    id: Uuid,
    default_role: String,
}

const ADD_USER_QUERY: &str = "
  INSERT INTO users (email, password_hash, name)
  VALUES ($1, $2, $3)
  RETURNING id, default_role";

pub async fn register(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let hashed_password = hash(credentials.password, 10)?;

    let user: LoggedInUser = sqlx::query_as(ADD_USER_QUERY)
        .bind(credentials.email)
        .bind(hashed_password)
        .bind(credentials.name)
        .fetch_one(&db)
        .await?;

    let user_session =
        jwt::session::create_session(&db, &jwt_secret, user.id, user.default_role).await?;

    Ok(Response::builder(200).body(json!(user_session)).build())
}
