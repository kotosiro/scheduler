mod domain;
mod services;
mod use_cases;
use crate::config::Config;
use crate::middlewares;
use anyhow::Context;
use anyhow::Result;
use lapin::Connection;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;

pub struct Controller {
    pub id: Uuid,
    pub db_pool: PgPool,
    pub mq_conn: Connection,
    pub config: Config,
}

impl Controller {
    pub async fn new(config: Config) -> Result<Arc<Self>> {
        let db_pool = middlewares::new_pg_pool(&config)
            .await
            .context("failed to create postgres connection pool")?;
        let mq_conn = middlewares::new_rmq_connection(&config)
            .await
            .context("failed to create rabbitmq connection")?;
        Ok(Arc::new(Controller {
            id: Uuid::new_v4(),
            db_pool,
            mq_conn,
            config,
        }))
    }

    pub async fn start(self: Arc<Self>) -> Result<()> {
        if self.config.no_auth {
            warn!("authorization is disabled, this is not recommended in production");
        }
        use_cases::bind(self)
            .await
            .context("failed to start API server")?;
        Ok(())
    }
}
