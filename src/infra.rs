mod postgres;
mod rabbitmq;
use crate::config::Config;
use anyhow::Result;
use lapin::Connection;
use sqlx::PgPool;

pub async fn pg_pool(config: &Config) -> Result<PgPool> {
    postgres::connect(&config.db_url).await
}

pub async fn rmq_connection(config: &Config) -> Result<Connection> {
    rabbitmq::connect(&config.mq_addr).await
}
