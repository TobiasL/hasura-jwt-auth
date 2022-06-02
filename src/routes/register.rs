use crate::db::users::create_user;
use crate::jwt::session::create_session;
use crate::state::State;
use bcrypt::hash;
use jwt_simple::prelude::*;
use surf;
use tide::convert::json;
use tide::{Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SendRegisterEmailPayload {
    email: String,
    id: String,
}

pub async fn register(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let jwt_expires_in_minutes = req.state().jwt_expires_in_minutes.clone();
    let refresh_expires_in_days = req.state().refresh_expires_in_days.clone();
    let post_register_url = req.state().post_register_url.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let hashed_password = hash(credentials.password, 10)?;

    let user = create_user(&db, &credentials.email, hashed_password, credentials.name).await?;

    // The user is not connected to an organisation directly after registration.
    let user_session = create_session(
        &db,
        &jwt_secret,
        &jwt_expires_in_minutes,
        &refresh_expires_in_days,
        &user.id,
        &user.default_role,
        &None,
    )
    .await?;

    match post_register_url {
        None => Ok(Response::builder(200).body(json!(user_session)).build()),
        Some(url) => {
            let payload = SendRegisterEmailPayload {
                email: credentials.email,
                id: user.id.to_string(),
            };

            surf::post(url).body_json(&payload)?.await?;

            Ok(Response::builder(200).body(json!(user_session)).build())
        }
    }
}
