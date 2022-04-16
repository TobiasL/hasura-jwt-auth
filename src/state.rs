use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
}
