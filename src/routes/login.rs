use crate::db::users::get_user;
use crate::db::users::UserRow;
use crate::jwt::session::create_session;
use crate::response::build_response;
use crate::state::State;
use actix_web::{error, web, Error, HttpResponse};
use bcrypt::verify;
use jwt_simple::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn login(credentials: web::Json<LoginCredentials>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    match get_user(&data.db, &data.table_conn, &credentials.email).await? {
        None => Ok(HttpResponse::Unauthorized().body("User not found")),
        Some(UserRow {
            password_hash,
            id,
            default_role,
            org_id,
        }) => {
            let valid = verify(&credentials.password, &password_hash)
                .map_err(|_err| error::ErrorInternalServerError("Error hashing password"))?;

            if !valid {
                return Ok(HttpResponse::Unauthorized().body("Wrong password"));
            }

            let user_session = create_session(
                &data.db,
                &data.jwt_secret,
                &data.jwt_expires_in_minutes,
                &data.refresh_expires_in_days,
                &id,
                &default_role,
                &org_id,
            )
            .await?;

            Ok(build_response(user_session))
        }
    }
}
