use crate::db::users::get_user;
use crate::db::users::UserRow;
use crate::jwt::session::create_session;
use crate::state::State;
use bcrypt::verify;
use jwt_simple::prelude::*;
use tide::convert::json;
use tide::{Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn login(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let table_conn = req.state().table_conn.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    match get_user(&db, &table_conn, &credentials.email).await? {
        None => Ok(Response::builder(401).body("User not found").build()),
        Some(UserRow {
            password_hash,
            id,
            default_role,
            org_id,
        }) => {
            let valid = verify(credentials.password, &password_hash)?;

            if !valid {
                return Ok(Response::builder(401).body("Wrong password").build());
            }

            let user_session =
                create_session(&db, &jwt_secret, &id, &default_role, &org_id).await?;

            Ok(Response::builder(200).body(json!(user_session)).build())
        }
    }
}
