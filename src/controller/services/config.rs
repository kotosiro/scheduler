use crate::messages::config::ConfigUpdate;
use anyhow::Context;
use anyhow::Result;
use lapin::options::BasicPublishOptions;
use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use lapin::BasicProperties;
use lapin::Channel;
use lapin::ExchangeKind;

const CONFIG_EXCHANGE: &str = "kotosiro.config";

pub async fn setup(chan: &Channel) -> Result<()> {
    chan.exchange_declare(
        CONFIG_EXCHANGE,
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

pub async fn notify(chan: &Channel, update: ConfigUpdate) -> Result<()> {
    chan.basic_publish(
        CONFIG_EXCHANGE,
        "",
        BasicPublishOptions::default(),
        &serde_json::to_vec(&update)?,
        BasicProperties::default(),
    )
    .await
    .context("failed to notify config update")?;
    Ok(())
}
