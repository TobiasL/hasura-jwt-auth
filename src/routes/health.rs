use crate::state;

pub async fn health(_req: tide::Request<state::State>) -> tide::Result {
    Ok(tide::Response::builder(200).build())
}
