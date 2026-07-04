use sqlx::{Pool, Postgres, postgres::PgPool};
use std::sync::OnceLock;

use crate::config::DbConfig;

pub mod sessions;
pub mod users;

type Conn = Pool<Postgres>;

pub static SQLX_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init(config: &DbConfig) {
    let sqlx_pool = PgPool::connect(&config.url)
        .await
        .expect("Database connection failed.");
    crate::db::SQLX_POOL
        .set(sqlx_pool)
        .expect("sqlx pool should be set")
}

pub fn pool() -> &'static PgPool {
    SQLX_POOL.get().expect("sqlx pool should be set")
}
