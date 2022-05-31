use crate::state;
use tide::{Request, Response, Result};

pub async fn health(_req: Request<state::State>) -> Result {
    Ok(Response::builder(200).build())
}
