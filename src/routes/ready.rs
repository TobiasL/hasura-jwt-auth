use crate::db::ready::is_database_ready;
use crate::state::State;
use actix_web::{web, Error, HttpResponse};

pub async fn ready(data: web::Data<State>) -> Result<HttpResponse, Error> {
    is_database_ready(&&data.db).await?;

    Ok(HttpResponse::Ok().finish())
}
