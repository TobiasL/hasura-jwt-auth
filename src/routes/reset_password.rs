use crate::db::reset_password::get_user_ticket;
use crate::db::reset_password::set_user_password;
use crate::db::reset_password::set_user_ticket;
use crate::db::reset_password::ResetUserRow;
use crate::db::users::get_user;
use crate::state::State;
use actix_web::{error, web, Error, HttpResponse};
use bcrypt::hash;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    email: String,
}

#[derive(Debug, Serialize)]
struct SendResetEmailPayload {
    email: String,
    ticket: String,
}

#[derive(Debug, Serialize)]
struct SendResetTicketPayload {
    ticket: String,
}

pub async fn reset(credentials: web::Json<LoginCredentials>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    match get_user(&data.db, &None, &credentials.email).await? {
        None => Ok(HttpResponse::Unauthorized().body("User not found")),
        Some(user) => {
            let ticket = set_user_ticket(&data.db, &user.id).await?;
            let ticket_payload = SendResetTicketPayload {
                ticket: ticket.to_string(),
            };

            match &data.post_reset_password_url {
                None => Ok(HttpResponse::Ok().json(&ticket_payload)),
                Some(url) => {
                    let payload = SendResetEmailPayload {
                        email: credentials.email.to_string(),
                        ticket: ticket.to_string(),
                    };

                    let client = reqwest::Client::new();

                    client
                        .post(url)
                        .json(&payload)
                        .send()
                        .await
                        .map_err(|_err| error::ErrorInternalServerError("Error posting to POST_RESET_PASSWORD_URL"))?;

                    Ok(HttpResponse::Ok().json(&ticket_payload))
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCredentials {
    ticket: Uuid,
    password: String,
}

#[derive(Debug, Serialize)]
struct SendSetEmailPayload {
    email: String,
}

pub async fn set(credentials: web::Json<SetCredentials>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let found_user = get_user_ticket(&data.db, credentials.ticket).await?;

    match found_user {
        None => Ok(HttpResponse::Unauthorized().body("Ticket not found")),
        Some(ResetUserRow { id, email }) => {
            let hashed_password = hash(&credentials.password, 10)
                .map_err(|_err| error::ErrorInternalServerError("Error hashing password"))?;

            set_user_password(&data.db, id, hashed_password).await?;

            match &data.post_set_password_url {
                None => Ok(HttpResponse::Ok().finish()),
                Some(url) => {
                    let payload = SendSetEmailPayload { email };

                    let client = reqwest::Client::new();

                    client
                        .post(url)
                        .json(&payload)
                        .send()
                        .await
                        .map_err(|_err| error::ErrorInternalServerError("Error posting to POST_SET_PASSWORD_URL"))?;

                    Ok(HttpResponse::Ok().finish())
                }
            }
        }
    }
}
