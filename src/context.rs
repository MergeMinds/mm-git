use crate::git;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Context {
    pub db: Pool<Postgres>,
    pub repo_manager: git::RepoManager,
}
