use tide::{Request, Response, Result};

use crate::state;

pub async fn refresh(_req: Request<state::State>) -> Result {
    Ok(Response::builder(201).build())
}
