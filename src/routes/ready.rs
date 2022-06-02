use crate::db::ready::is_database_ready;
use crate::state::State;
use tide::{Request, Response, Result};

pub async fn ready(req: Request<State>) -> Result {
    let db = req.state().db.clone();

    is_database_ready(&db).await?;

    Ok(Response::builder(200).build())
}
