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
    pub mq_addr: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::env;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;
    use testutils;

    #[test]
    fn test_load_some() {
        let db_url: String = testutils::rand::url();
        let controller_addr: String = testutils::rand::ip();
        let controller_bind: String = testutils::rand::ip();
        let cluster_gossip_addr: String = testutils::rand::ip();
        let cluster_gossip_bind: String = testutils::rand::ip();
        let mq_addr: String = testutils::rand::ip();
        let use_json_log: bool = testutils::rand::bool();
        let log_filter: String = testutils::rand::string(20);

        let data = format!(
            r#"db_url = "{db_url}"
controller_addr = "{controller_addr}"
controller_bind = "{controller_bind}"
cluster_gossip_addr = "{cluster_gossip_addr}"
cluster_gossip_bind = "{cluster_gossip_bind}"
mq_addr = "{mq_addr}"
use_json_log = {use_json_log}
log_filter = "{log_filter}""#
        );

        println!("{}", data);
        let file = match testutils::io::tempfile(&data) {
            Ok(file) => file,
            Err(e) => panic!("could not create test configuration file: {e}"),
        };

        let path = Path::new("./config_test.toml");
        let file = match file.persist(&path) {
            Ok(file) => file,
            Err(e) => panic!("could not persist test configuration file: {e}"),
        };

        let conf = match load(Some(&path)) {
            Ok(conf) => conf,
            Err(e) => panic!("config object must be loaded: {e}"),
        };

        assert_eq!(db_url, conf.db_url);
        assert_eq!(controller_addr, conf.controller_addr);
        assert_eq!(controller_bind, conf.controller_bind);
        assert_eq!(cluster_gossip_addr, conf.cluster_gossip_addr);
        assert_eq!(cluster_gossip_bind, conf.cluster_gossip_bind);
        assert_eq!(mq_addr, conf.mq_addr);
        assert_eq!(use_json_log, conf.use_json_log);
        assert_eq!(log_filter, conf.log_filter);

        remove_file(&path);
    }

    #[test]
    fn test_load_none() {
        let db_url: String = testutils::rand::url();
        let controller_addr: String = testutils::rand::ip();
        let controller_bind: String = testutils::rand::ip();
        let cluster_gossip_addr: String = testutils::rand::ip();
        let cluster_gossip_bind: String = testutils::rand::ip();
        let mq_addr: String = testutils::rand::ip();
        let use_json_log: bool = testutils::rand::bool();
        let log_filter: String = testutils::rand::string(20);

        env::set_var("KOTOSIRO_DB_URL", &db_url);
        env::set_var("KOTOSIRO_CONTROLLER_ADDR", &controller_addr);
        env::set_var("KOTOSIRO_CONTROLLER_BIND", &controller_bind);
        env::set_var("KOTOSIRO_CLUSTER_GOSSIP_ADDR", &cluster_gossip_addr);
        env::set_var("KOTOSIRO_CLUSTER_GOSSIP_BIND", &cluster_gossip_bind);
        env::set_var("KOTOSIRO_MQ_ADDR", &mq_addr);
        env::set_var("KOTOSIRO_USE_JSON_LOG", use_json_log.to_string());
        env::set_var("KOTOSIRO_LOG_FILTER", &log_filter);

        let conf = match load(None) {
            Ok(conf) => conf,
            Err(e) => panic!("config object must be loaded: {e}"),
        };

        assert_eq!(db_url, conf.db_url);
        assert_eq!(controller_addr, conf.controller_addr);
        assert_eq!(controller_bind, conf.controller_bind);
        assert_eq!(cluster_gossip_addr, conf.cluster_gossip_addr);
        assert_eq!(cluster_gossip_bind, conf.cluster_gossip_bind);
        assert_eq!(mq_addr, conf.mq_addr);
        assert_eq!(use_json_log, conf.use_json_log);
        assert_eq!(log_filter, conf.log_filter);

        env::remove_var("KOTOSIRO_DB_URL");
        env::remove_var("KOTOSIRO_CONTROLLER_ADDR");
        env::remove_var("KOTOSIRO_CONTROLLER_BIND");
        env::remove_var("KOTOSIRO_CLUSTER_GOSSIP_ADDR");
        env::remove_var("KOTOSIRO_CLUSTER_GOSSIP_BIND");
        env::remove_var("KOTOSIRO_MQ_ADDR");
        env::remove_var("KOTOSIRO_USE_JSON_LOG");
        env::remove_var("KOTOSIRO_LOG_FILTER");
    }
}
