use actix_web::{error, Error};
use sqlx::PgPool;

const CHECK_DATABASE_CONNECTION: &str = "SELECT 1 FROM users LIMIT 1;";

pub async fn is_database_ready(db: &PgPool) -> Result<(), Error> {
    sqlx::query(CHECK_DATABASE_CONNECTION)
        .execute(db)
        .await
        .map_err(|_err| error::ErrorInternalServerError("Error querying database"))?;

    Ok(())
}
