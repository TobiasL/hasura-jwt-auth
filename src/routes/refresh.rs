use crate::db::refresh_tokens::get_and_delete_refresh_token;
use crate::db::refresh_tokens::RefreshUserRow;
use crate::jwt::session::create_session;
use crate::response::build_response;
use crate::state::State;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use jwt_simple::prelude::*;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshPayload {
    refresh: Uuid,
}

fn get_refresh_cookie(req: HttpRequest) -> Result<Uuid, Error> {
    let refresh_cookie = req.cookie("refresh");

    match refresh_cookie {
        None => Err(error::ErrorUnauthorized("Refresh token not found")),
        Some(refresh_value) => {
            let refresh = Uuid::parse_str(refresh_value.value())
                .map_err(|_err| error::ErrorUnauthorized("Malformed refresh token"))?;

            Ok(refresh)
        }
    }
}

pub async fn refresh(req: HttpRequest, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let refresh_uuid = get_refresh_cookie(req)?;

    match get_and_delete_refresh_token(&data.db, &data.table_conn, &refresh_uuid).await? {
        None => Err(error::ErrorUnauthorized("Refresh token not found")),
        Some(RefreshUserRow {
            user_id,
            default_role,
            org_id,
        }) => {
            let user_session = create_session(
                &data.db,
                &data.jwt_secret,
                &data.jwt_expires_in_minutes,
                &data.refresh_expires_in_days,
                &user_id,
                &default_role,
                &org_id,
            )
            .await?;

            Ok(build_response(user_session))
        }
    }
}
