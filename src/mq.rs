use crate::config::Config;
use anyhow::Result;
use lapin::Connection;
use lapin::ConnectionProperties;
use tracing::info;

async fn lapin_connect(addr: &str) -> Result<Connection> {
    info!("connecting to message broker");
    let uri = addr.parse().map_err(anyhow::Error::msg)?;
    let conn = Connection::connect_uri(uri, ConnectionProperties::default()).await?;
    info!("connected to message broker");
    Ok(conn)
}

pub async fn connect(config: &Config) -> Result<Connection> {
    lapin_connect(&config.mq_addr).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use lapin::options::BasicConsumeOptions;
    use lapin::options::BasicPublishOptions;
    use lapin::options::ExchangeDeclareOptions;
    use lapin::options::QueueBindOptions;
    use lapin::options::QueueDeclareOptions;
    use lapin::types::FieldTable;
    use lapin::BasicProperties;
    use lapin::ExchangeKind;
    use std::time::Duration;
    use testcontainers::clients;
    use testcontainers::images::rabbitmq;
    use testutils;

    #[tokio::test]
    async fn test_connect() {
        let docker = clients::Cli::default();
        let node = docker.run(rabbitmq::RabbitMq::default());
        let url = format!("amqp://127.0.0.1:{}", node.get_host_port_ipv4(5672));

        let conn = match lapin_connect(&url).await {
            Ok(conn) => conn,
            Err(e) => panic!("could not create connection: {e}"),
        };

        let channel = match conn.create_channel().await {
            Ok(channel) => channel,
            Err(e) => panic!("could not channel: {e}"),
        };

        assert!(channel.status().connected());

        let exchange_name = testutils::rand::string(20);
        let queue_name = testutils::rand::string(20);
        let consumer_tag = testutils::rand::string(20);
        let routing_key = testutils::rand::string(20);
        let payload = testutils::rand::bytes(20);

        match channel
            .exchange_declare(
                &exchange_name,
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("could not declare channel exchange: {e}"),
        };

        let queue = match channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(channel) => channel,
            Err(e) => panic!("could not declare queue: {e}"),
        };

        match channel
            .exchange_declare(
                &exchange_name,
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("could not declare channel exchange: {e}"),
        };

        match channel
            .queue_bind(
                queue.name().as_str(),
                &exchange_name,
                "#",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("could not bind queue: {e}"),
        };

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                &consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        match channel
            .basic_publish(
                &exchange_name,
                &routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => panic!("could not publish: {e}"),
        };

        let consumed = match tokio::time::timeout(Duration::from_secs(10), consumer.next()).await {
            Ok(Some(consumed)) => consumed,
            _ => panic!("could not generate timeout"),
        };

        let delivery = consumed.expect("Failed to consume delivery!");
        assert_eq!(&delivery.data.clone().to_vec(), &payload);
        assert_eq!(&delivery.exchange.as_str(), &exchange_name);
        assert_eq!(&delivery.routing_key.as_str(), &routing_key);
    }
}
