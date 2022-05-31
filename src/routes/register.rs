use crate::db::users::create_user;
use crate::jwt::session::create_session;
use crate::state::State;
use bcrypt::hash;
use jwt_simple::prelude::*;
use tide::convert::json;
use tide::{Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    name: String,
    email: String,
    password: String,
}

pub async fn register(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let hashed_password = hash(credentials.password, 10)?;

    let user = create_user(&db, credentials.email, hashed_password, credentials.name).await?;

    // The user is not connected to an organisation directly after registration.
    let user_session = create_session(&db, &jwt_secret, &user.id, &user.default_role, &None).await?;

    Ok(Response::builder(200).body(json!(user_session)).build())
}
