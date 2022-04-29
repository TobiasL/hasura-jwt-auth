use crate::jwt;
use crate::state;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use tide::convert::json;
use tide::{Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct RefreshPayload {
    refresh: Uuid,
}

pub async fn refresh(mut req: Request<state::State>) -> Result {
    let db = req.state().db.clone();
    let jwt_secret = req.state().jwt_secret.clone();
    let credentials: RefreshPayload = req.body_json().await?;

    let user = jwt::refresh::get_refresh_token(&db, credentials.refresh).await?;
    let user_session =
        jwt::session::create_session(&db, &jwt_secret, user.user_id, user.default_role).await?;

    Ok(Response::builder(200).body(json!(user_session)).build())
}
