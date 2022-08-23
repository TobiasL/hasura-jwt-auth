use crate::db::init::OrgTableInfo;
use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
    pub jwt_secret: String,
    pub table_conn: Option<OrgTableInfo>,
    pub jwt_expires_in_minutes: u64,
    pub refresh_expires_in_days: u64,
    pub post_register_url: Option<String>,
    pub post_reset_password_url: Option<String>,
    pub post_set_password_url: Option<String>,
}
