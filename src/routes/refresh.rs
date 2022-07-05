use crate::db::refresh_tokens::get_and_delete_refresh_token;
use crate::db::refresh_tokens::RefreshUserRow;
use crate::jwt::session::create_session;
use crate::state::State;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use tide::convert::json;
use tide::{Error, Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct RefreshPayload {
    refresh: Uuid,
}

pub async fn refresh(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let table_conn = req.state().table_conn.clone();
    let jwt_expires_in_minutes = req.state().jwt_expires_in_minutes.clone();
    let refresh_expires_in_days = req.state().refresh_expires_in_days.clone();
    let credentials: RefreshPayload = req.body_json().await?;

    match get_and_delete_refresh_token(&db, &table_conn, &credentials.refresh).await? {
        None => Err(Error::from_str(401, "Refresh token not found")),
        Some(RefreshUserRow {
            user_id,
            default_role,
            org_id,
        }) => {
            let user_session = create_session(
                &db,
                &jwt_secret,
                &jwt_expires_in_minutes,
                &refresh_expires_in_days,
                &user_id,
                &default_role,
                &org_id,
            )
            .await?;

            Ok(Response::builder(200).body(json!(user_session)).build())
        }
    }
}
