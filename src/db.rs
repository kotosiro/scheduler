use crate::config::Config;
use anyhow::Result;
use sqlx::Executor;
use sqlx::PgPool;
use tracing::info;
use tracing::trace;

async fn sqlx_connect(db_url: &str) -> Result<PgPool> {
    info!("connecting to database");
    let pool = PgPool::connect(&db_url).await?;
    let mut conn = pool.acquire().await?;
    let done = conn.execute(include_str!("db/schema.sql")).await?;
    trace!("schema created: {} rows modified", done.rows_affected());
    info!("connected to database");
    Ok(pool)
}

pub async fn connect(config: &Config) -> Result<PgPool> {
    sqlx_connect(&config.db_url).await
}
