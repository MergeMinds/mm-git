use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Context {
    pub db: Pool<Postgres>,
}
