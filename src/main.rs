mod db;
mod jwt;
mod routes;
mod state;

use db::init::check_org_column;
use db::init::connect_and_migrate;
use routes::live::live;
use routes::login::login;
use routes::ready::ready;
use routes::refresh::refresh;
use routes::register::register;
use routes::reset_password::{reset, set};
use std::env;

fn get_listen_address() -> String {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("80".to_string());

    format!("{host}:{port}")
}

#[test]
fn test_default_listen_address() -> Result<(), String> {
    assert_eq!(get_listen_address(), "0.0.0.0:80");

    Ok(())
}

fn get_jwt_expires() -> u64 {
    let jwt_expires_in_minutes = env::var("JWT_EXPIRES_IN_MINUTES").ok();

    jwt_expires_in_minutes.map_or(15, |value| value.parse::<u64>().unwrap_or(15))
}

fn get_refresh_expires() -> u64 {
    let refresh_expires_in_days = env::var("REFRESH_EXPIRES_IN_DAYS").ok();

    refresh_expires_in_days.map_or(60, |value| value.parse::<u64>().unwrap_or(60))
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let jwt_secret = env::var("JWT_SECRET").expect("Env variable JWT_SECRET is not set");
    let db_url = env::var("DATABASE_URL").expect("Env variable DATABASE_URL is not set");

    let database_connections = env::var("DATABASE_CONNECTIONS").ok();
    let post_register_url = env::var("POST_REGISTER_URL").ok();
    let post_reset_password_url = env::var("POST_RESET_PASSWORD_URL").ok();
    let post_set_password_url = env::var("POST_SET_PASSWORD_URL").ok();
    let org_table_column = env::var("JWT_ORG_CUSTOM_CLAIM").ok();

    let pg_pool = connect_and_migrate(&db_url, database_connections).await?;
    let table_conn = check_org_column(&pg_pool, org_table_column).await?;

    let mut app = tide::with_state(state::State {
        db: pg_pool,
        jwt_secret,
        table_conn,
        post_register_url,
        post_reset_password_url,
        post_set_password_url,
        jwt_expires_in_minutes: get_jwt_expires(),
        refresh_expires_in_days: get_refresh_expires(),
    });

    app.at("/livez").get(live);
    app.at("/readyz").get(ready);

    app.at("/register").post(register);
    app.at("/login").post(login);
    app.at("/refresh").post(refresh);
    app.at("/reset-password").post(reset);
    app.at("/password").post(set);

    let listen_address = get_listen_address();

    app.listen(listen_address).await?;

    Ok(())
}
