use sqlx::postgres::PgPoolOptions;
use std::env;

mod auth;
mod health;
mod sign;
mod state;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let db_url = "postgres://postgres:postgrespassword@127.0.0.1:5432/local_db";

    let mut app = tide::with_state(state::State {
        db: PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?,
    });

    // TODO: Exit more cleanly if it's not set.
    env::var("JWT_SECRET").unwrap();

    app.at("/health").get(health::health);
    app.at("/sign").get(sign::sign);
    app.at("/login").post(auth::login);

    // TODO: Add the ability to override the port with env variable.
    app.listen("127.0.0.1:4444").await?;

    Ok(())
}
