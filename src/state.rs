use crate::db::init::TableConn;
use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
    pub jwt_secret: String,
    pub table_conn: Option<TableConn>,
    pub post_reset_password_url: Option<String>,
}
