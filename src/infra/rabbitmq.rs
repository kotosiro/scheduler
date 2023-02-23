use anyhow::Result;
use lapin::Connection;
use lapin::ConnectionProperties;
use tracing::info;

pub async fn connect(addr: &str) -> Result<Connection> {
    info!("connecting to message broker");
    let uri = addr.parse().map_err(anyhow::Error::msg)?;
    let conn = Connection::connect_uri(uri, ConnectionProperties::default()).await?;
    info!("connected to message broker");
    Ok(conn)
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

    #[tokio::test]
    #[ignore]
    async fn test_connect() {
        let docker = clients::Cli::default();
        let node = docker.run(rabbitmq::RabbitMq::default());
        let url = format!("amqp://127.0.0.1:{}", node.get_host_port_ipv4(5672));

        let conn = connect(&url)
            .await
            .expect("connection should be established");

        let channel = conn
            .create_channel()
            .await
            .expect("channel should be created");

        assert!(channel.status().connected());

        let exchange_name = testutils::rand::string(20);
        let queue_name = testutils::rand::string(20);
        let consumer_tag = testutils::rand::string(20);
        let routing_key = testutils::rand::string(20);
        let payload = testutils::rand::bytes(20);

        channel
            .exchange_declare(
                &exchange_name,
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("exchange should be declared");

        let queue = channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue should be declared");

        channel
            .exchange_declare(
                &exchange_name,
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("exchange should be decraled");

        channel
            .queue_bind(
                queue.name().as_str(),
                &exchange_name,
                "#",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue should be binded");

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                &consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("comsumer should be created");

        channel
            .basic_publish(
                &exchange_name,
                &routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .expect("channel should be published");

        let consumed = tokio::time::timeout(Duration::from_secs(10), consumer.next())
            .await
            .expect("timeout should be declared")
            .expect("timeout should be consumed");

        let delivery = consumed.expect("Failed to consume delivery!");

        assert_eq!(&delivery.data.clone().to_vec(), &payload);
        assert_eq!(&delivery.exchange.as_str(), &exchange_name);
        assert_eq!(&delivery.routing_key.as_str(), &routing_key);
    }
}
