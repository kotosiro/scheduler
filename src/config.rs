use config::{builder::DefaultState, ConfigBuilder, Environment, File, FileFormat};

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub db_url: String,
    pub controller_addr: String,
    pub controller_bind: String,
    pub cluster_gossip_bind: String,
    pub cluster_gossip_addr: String,
}

fn loader(file: Option<&Path>) -> ConfigBuilder<DefaultState> {
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
            .list_separator(",")
            .try_parsing(true),
    )
}

pub fn load(file: Option<&Path>) -> Result<Config> {
    let config = loader(file)
        .build()?
        .try_deserialize()
        .context("mandatory configuration value not set")?;
    Ok(config)
}
