use bcrypt::verify;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;

use crate::jwt;
use crate::state;

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct LoggedInUser {
    id: Uuid,
    password_hash: String,
    default_role: String,
}

const GET_USER_QUERY: &str = "SELECT * from users where email = $1";

pub async fn login(mut req: tide::Request<state::State>) -> tide::Result {
    let db = req.state().db.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let user: LoggedInUser = sqlx::query_as(GET_USER_QUERY)
        .bind(credentials.email)
        .fetch_one(&db)
        .await?;

    let valid = verify(credentials.password, &user.password_hash)?;

    if !valid {
        return Ok(tide::Response::builder(401).body("Wrong password").build());
    }

    let token = jwt::token::create_token(user.id, user.default_role)?;

    Ok(tide::Response::builder(200).body(token).build())
}
