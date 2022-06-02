use crate::state::State;
use tide::{Request, Response, Result};

pub async fn live(_req: Request<State>) -> Result {
    Ok(Response::builder(200).build())
}
