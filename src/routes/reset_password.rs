use bcrypt::hash;
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use surf;
use tide::convert::json;
use tide::{Request, Response, Result};

use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
struct LoginCredentials {
    email: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct TicketRow {
    ticket: Uuid,
}

const GET_USER_QUERY: &str = "
    SELECT id FROM users WHERE email = $1;
";

const SET_USER_TICKET_QUERY: &str = "
    UPDATE users
    SET ticket = public.gen_random_uuid(), ticket_expires_at = current_timestamp + interval '1 h'
    WHERE id = $1 RETURNING ticket;
";

#[derive(Debug, Serialize)]
struct SendEmailPayload {
    email: String,
    ticket: String,
}

pub async fn reset(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let post_reset_password_url = req.state().post_reset_password_url.clone();
    let credentials: LoginCredentials = req.body_json().await?;

    let found_user: Option<UserRow> = sqlx::query_as(GET_USER_QUERY)
        .bind(&credentials.email)
        .fetch_optional(&db)
        .await?;

    match found_user {
        None => Ok(Response::builder(401).body("User not found").build()),
        Some(user) => {
            let ticket: TicketRow = sqlx::query_as(SET_USER_TICKET_QUERY)
                .bind(user.id)
                .fetch_one(&db)
                .await?;

            match post_reset_password_url {
                None => Ok(Response::builder(200).body(json!(ticket)).build()),
                Some(url) => {
                    let payload = SendEmailPayload {
                        email: credentials.email,
                        ticket: ticket.ticket.to_string(),
                    };

                    surf::post(url).body_json(&payload)?.await?;

                    Ok(Response::builder(200).body(json!(ticket)).build())
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

const GET_TICKET_USER_QUERY: &str = "
    SELECT id FROM users
    WHERE ticket_expires_at > CURRENT_TIMESTAMP AND ticket = $1;
";

const SET_NEW_PASSWORD_QUERY: &str = "
    UPDATE users
    SET password_hash = $1
    WHERE id = $2;
";

pub async fn set(mut req: Request<State>) -> Result {
    let db = req.state().db.clone();
    let credentials: SetCredentials = req.body_json().await?;

    let found_user: Option<UserRow> = sqlx::query_as(GET_TICKET_USER_QUERY)
        .bind(credentials.ticket)
        .fetch_optional(&db)
        .await?;

    match found_user {
        None => Ok(Response::builder(401).body("Ticket not found").build()),
        Some(user) => {
            let hashed_password = hash(credentials.password, 10)?;

            sqlx::query(SET_NEW_PASSWORD_QUERY)
                .bind(hashed_password)
                .bind(user.id)
                .execute(&db)
                .await?;

            Ok(Response::builder(200).build())
        }
    }
}
