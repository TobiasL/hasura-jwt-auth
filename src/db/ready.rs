use sqlx::PgPool;
use tide::Result;

const CHECK_DATABASE_CONNECTION: &str = "SELECT 1 FROM users LIMIT 1;";

pub async fn is_database_ready(db: &PgPool) -> Result<()> {
    sqlx::query(CHECK_DATABASE_CONNECTION).execute(db).await?;

    Ok(())
}
