use sqlx::{types::Json, types::Uuid};
use tide::prelude::*;

use super::state;

#[derive(Deserialize, Serialize)]
struct Tags {
    tag_id: Uuid,
    name: String,
}

#[derive(Serialize, sqlx::Type, sqlx::FromRow)]
struct Articles {
    article_id: Uuid,
    headline: String,
    tags: Json<Vec<Tags>>,
}

const ALL_ARTICLES_QUERY: &str = "select * from articles_with_tags";

pub async fn login(req: tide::Request<state::State>) -> tide::Result {
    let state = req.state();

    let rows: Vec<Articles> = sqlx::query_as(ALL_ARTICLES_QUERY)
        .fetch_all(&state.db)
        .await?;

    Ok(tide::Response::builder(200).body(json!(rows)).build())
}
