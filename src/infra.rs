pub mod jwt;
pub mod opa;
pub mod postgres;
pub mod rabbitmq;
use crate::config::Config;
use anyhow::Result;
use lapin::Connection;
use sqlx::PgPool;

pub async fn new_pg_pool(config: &Config) -> Result<PgPool> {
    postgres::connect(&config.db_url).await
}

pub async fn new_rmq_connection(config: &Config) -> Result<Connection> {
    rabbitmq::connect(&config.mq_addr).await
}
