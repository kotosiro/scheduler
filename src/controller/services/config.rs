use crate::messages::config::ConfigUpdate;
use crate::messages::config::CONFIG_UPDATES_EXCHANGE;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use lapin::options::BasicPublishOptions;
use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use lapin::BasicProperties;
use lapin::Channel;
use lapin::ExchangeKind;

#[async_trait]
pub trait ConfigService {
    async fn setup(&self) -> Result<()>;

    async fn publish(&self, update: ConfigUpdate) -> Result<()>;
}

#[async_trait]
impl ConfigService for Channel {
    async fn setup(&self) -> Result<()> {
        self.exchange_declare(
            CONFIG_UPDATES_EXCHANGE,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..ExchangeDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await
        .context("failed to declare rabbitmq exchange")?;
        Ok(())
    }

    async fn publish(&self, update: ConfigUpdate) -> Result<()> {
        self.basic_publish(
            CONFIG_UPDATES_EXCHANGE,
            "",
            BasicPublishOptions::default(),
            &serde_json::to_vec(&update)?,
            BasicProperties::default(),
        )
        .await
        .context("failed to notify config update")?;
        Ok(())
    }
}
