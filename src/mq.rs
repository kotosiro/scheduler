use crate::config::Config;
use anyhow::Result;
use lapin::Connection;
use lapin::ConnectionProperties;
use tracing::info;

pub async fn connect(config: &Config) -> Result<Connection> {
    info!("connecting to message broker");
    let addr = &config.mq_addr;
    let uri = addr.parse().map_err(anyhow::Error::msg)?;
    let conn = Connection::connect_uri(uri, ConnectionProperties::default()).await?;
    info!("connected to message broker");
    Ok(conn)
}
