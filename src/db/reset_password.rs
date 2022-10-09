use actix_web::{error, Error};
use jwt_simple::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct TicketRow {
    ticket: Uuid,
}

const SET_USER_TICKET_QUERY: &str = "
    UPDATE users
    SET ticket = public.gen_random_uuid(), ticket_expires_at = current_timestamp + interval '1 h'
    WHERE id = $1 RETURNING ticket;
";

pub async fn set_user_ticket(db: &PgPool, user_id: &Uuid) -> Result<Uuid, Error> {
    let ticket: TicketRow = sqlx::query_as(SET_USER_TICKET_QUERY)
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(ticket.ticket)
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ResetUserRow {
    pub id: Uuid,
    pub email: String,
}

const GET_TICKET_USER_QUERY: &str = "
    SELECT id, email::text FROM users
    WHERE ticket_expires_at > CURRENT_TIMESTAMP AND ticket = $1;
";

pub async fn get_user_ticket(db: &PgPool, ticket: Uuid) -> Result<Option<ResetUserRow>, Error> {
    let found_user: Option<ResetUserRow> = sqlx::query_as(GET_TICKET_USER_QUERY)
        .bind(ticket)
        .fetch_optional(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(found_user)
}

const SET_NEW_PASSWORD_QUERY: &str = "
    UPDATE users
    SET password_hash = $1
    WHERE id = $2;
";

pub async fn set_user_password(db: &PgPool, user_id: Uuid, hashed_password: String) -> Result<(), Error> {
    sqlx::query(SET_NEW_PASSWORD_QUERY)
        .bind(hashed_password)
        .bind(user_id)
        .execute(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(())
}
