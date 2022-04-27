use sqlx::postgres::PgPoolOptions;
use std::env;

mod auth;
mod health;
mod sign;
mod state;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let db_url = env::var("DATABASE_URL").map_err(|_err| format!("Env variable DATABASE_URL is not set")).unwrap();

    let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
    let mut app = tide::with_state(state::State {
        db: pg_pool,
    });

    env::var("JWT_SECRET").map_err(|_err| format!("Env variable JWT_SECRET is not set")).unwrap();

    app.at("/health").get(health::health);
    app.at("/sign").get(sign::sign);
    app.at("/login").post(auth::login);

    // TODO: Add the ability to override the port with env variable.
    app.listen("0.0.0.0:4444").await?;

    Ok(())
}
