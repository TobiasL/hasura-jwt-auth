use tide::{Request, Response, Result};

use crate::state;

pub async fn health(_req: Request<state::State>) -> Result {
    Ok(Response::builder(200).build())
}
