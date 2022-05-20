use sqlx::postgres::PgPoolOptions;
use std::env;

mod jwt;
mod routes;
mod state;
mod user_org;

fn get_listen_address() -> String {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("80".to_string());

    format!("{host}:{port}")
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let jwt_secret = env::var("JWT_SECRET").expect("Env variable JWT_SECRET is not set");
    let db_url = env::var("DATABASE_URL").expect("Env variable DATABASE_URL is not set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pg_pool).await?;
    let table_conn = user_org::check_org_column(&pg_pool).await?;

    let mut app = tide::with_state(state::State {
        db: pg_pool,
        jwt_secret,
        table_conn,
    });

    app.at("/health").get(routes::health::health);

    app.at("/register").post(routes::register::register);
    app.at("/login").post(routes::login::login);
    app.at("/refresh").post(routes::refresh::refresh);

    app.at("/reset-password")
        .post(routes::reset_password::reset);
    app.at("/password").post(routes::reset_password::set);

    let listen_address = get_listen_address();

    app.listen(listen_address).await?;

    Ok(())
}
