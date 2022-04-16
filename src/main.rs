use sqlx::postgres::PgPoolOptions;

mod auth;
mod state;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let db_url = "postgres://postgres:password@localhost:5432/auth_db";

    let mut app = tide::with_state(state::State {
        db: PgPoolOptions::new().max_connections(5).connect(db_url).await?
    });

    app.at("/login").post(auth::login);

    app.listen("127.0.0.1:4040").await?;

    Ok(())
}
