use crate::db::reset_password::get_user_ticket;
use crate::db::reset_password::set_user_password;
use crate::db::reset_password::set_user_ticket;
use crate::db::reset_password::ResetUserRow;
use crate::db::users::get_user;
use crate::state::State;
use bcrypt::hash;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use surf;
use tide::convert::json;
use tide::{Request, Response, Result};

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
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

pub async fn reset(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let post_reset_password_url = req.state().post_reset_password_url.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    match get_user(&db, &None, &credentials.email).await? {
        None => Ok(Response::builder(401).body("User not found").build()),
        Some(user) => {
            let ticket = set_user_ticket(&db, &user.id).await?;
            let ticket_payload = SendResetTicketPayload {
                ticket: ticket.to_string(),
            };
            match post_reset_password_url {
                None => Ok(Response::builder(200).body(json!(&ticket_payload)).build()),
                Some(url) => {
                    let payload = SendResetEmailPayload {
                        email: credentials.email,
                        ticket: ticket.to_string(),
                    };

                    surf::post(url).body_json(&payload)?.await?;

                    Ok(Response::builder(200).body(json!(&ticket_payload)).build())
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SetCredentials {
    ticket: Uuid,
    password: String,
}

#[derive(Debug, Serialize)]
struct SendSetEmailPayload {
    email: String,
}

pub async fn set(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let post_set_password_url = req.state().post_set_password_url.clone();
    let credentials: SetCredentials = req.body_json().await?;

    let found_user = get_user_ticket(&db, credentials.ticket).await?;

    match found_user {
        None => Ok(Response::builder(401).body("Ticket not found").build()),
        Some(ResetUserRow { id, email }) => {
            let hashed_password = hash(credentials.password, 10)?;

            set_user_password(&db, id, hashed_password).await?;

            match post_set_password_url {
                None => Ok(Response::builder(200).build()),
                Some(url) => {
                    let payload = SendSetEmailPayload { email };

                    surf::post(url).body_json(&payload)?.await?;

                    Ok(Response::builder(200).build())
                }
            }
        }
    }
}
