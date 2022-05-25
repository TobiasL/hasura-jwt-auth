use sqlx::PgPool;

use crate::user_org::TableConn;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
    pub jwt_secret: String,
    pub table_conn: Option<TableConn>,
    pub post_reset_password_url: Option<String>,
}
