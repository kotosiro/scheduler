use anyhow::Context;
use anyhow::Result;
use config::builder::DefaultState;
use config::ConfigBuilder;
use config::Environment;
use config::File;
use config::FileFormat;
use std::path::Path;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub controller_addr: String,
    pub controller_bind: String,
    pub cluster_gossip_bind: String,
    pub cluster_gossip_addr: String,
    pub use_json_log: bool,
    pub log_filter: String,
}

fn builder(file: Option<&Path>) -> ConfigBuilder<DefaultState> {
    let mut builder = config::Config::builder();

    builder = builder.add_source(File::from_str(
        include_str!("etc/defaults.toml"),
        FileFormat::Toml,
    ));

    if let Some(file) = file {
        builder = builder.add_source(File::from(file));
    }

    builder.add_source(
        Environment::with_prefix("KOTOSIRO")
            //.list_separator(",")
            .try_parsing(true),
    )
}

pub fn load(file: Option<&Path>) -> Result<Config> {
    let config = builder(file)
        .build()?
        .try_deserialize()
        .context("mandatory configuration value not set")?;
    Ok(config)
}
