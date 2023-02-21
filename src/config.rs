mod builder;
use anyhow::Context;
use anyhow::Result;
use std::path::Path;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub controller_addr: String,
    pub controller_bind: String,
    pub cluster_gossip_bind: String,
    pub cluster_gossip_addr: String,
    pub mq_addr: String,
    pub use_json_log: bool,
    pub log_filter: String,
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Config> {
        let config = builder::new(path)
            .build()?
            .try_deserialize()
            .context("mandatory configuration value not set")?;
        Ok(config)
    }
}
