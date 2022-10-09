use crate::db::users::create_user;
use crate::jwt::session::create_session;
use crate::response::build_response;
use crate::state::State;
use actix_web::{error, web, Error, HttpResponse};
use bcrypt::hash;
use jwt_simple::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SendRegisterEmailPayload {
    email: String,
    id: String,
}

pub async fn register(credentials: web::Json<LoginCredentials>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let hashed_password =
        hash(&credentials.password, 10).map_err(|_err| error::ErrorInternalServerError("Error hashing password"))?;

    let user = create_user(&data.db, &credentials.email, &hashed_password, &credentials.name).await?;

    // The user is not connected to an organisation directly after registration.
    let user_session = create_session(
        &data.db,
        &data.jwt_secret,
        &data.jwt_expires_in_minutes,
        &data.refresh_expires_in_days,
        &user.id,
        &user.default_role,
        &None,
    )
    .await?;

    match &data.post_register_url {
        None => Ok(build_response(user_session)),
        Some(url) => {
            let payload = SendRegisterEmailPayload {
                email: credentials.email.to_string(),
                id: user.id.to_string(),
            };

            let client = reqwest::Client::new();

            client
                .post(url)
                .json(&payload)
                .send()
                .await
                .map_err(|_err| error::ErrorInternalServerError("Error posting to POST_REGISTER_URL"))?;

            Ok(build_response(user_session))
        }
    }
}
