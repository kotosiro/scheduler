use config::builder::DefaultState;
use config::ConfigBuilder;
use config::Environment;
use config::File;
use config::FileFormat;
use std::path::Path;

pub fn new(path: Option<&Path>) -> ConfigBuilder<DefaultState> {
    let mut builder = config::Config::builder();
    builder = builder.add_source(File::from_str(
        include_str!("defaults.toml"),
        FileFormat::Toml,
    ));
    if let Some(path) = path {
        builder = builder.add_source(File::from(path));
    }
    builder.add_source(
        Environment::with_prefix("KOTOSIRO")
            //.list_separator(",")
            .try_parsing(true),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::path::Path;

    #[test]
    #[serial]
    #[ignore]
    fn test_new_some() {
        let db_url: String = testutils::rand::url();
        let controller_addr: String = testutils::rand::ip();
        let controller_bind: String = testutils::rand::ip();
        let cluster_gossip_addr: String = testutils::rand::ip();
        let cluster_gossip_bind: String = testutils::rand::ip();
        let mq_addr: String = testutils::rand::ip();
        let use_json_log: bool = testutils::rand::bool();
        let log_filter: String = testutils::rand::string(20);
        let config = format!(
            include_str!("config.tmpl"),
            db_url = &db_url,
            controller_addr = &controller_addr,
            controller_bind = &controller_bind,
            cluster_gossip_addr = &cluster_gossip_addr,
            cluster_gossip_bind = &cluster_gossip_bind,
            mq_addr = &mq_addr,
            use_json_log = &use_json_log,
            log_filter = &log_filter
        );
        let path = testutils::io::persist(&config, Path::new("./config.toml"))
            .expect("path should be created");
        let config: crate::config::Config = new(Some(&path))
            .build()
            .expect("builder should be able to build configuration")
            .try_deserialize()
            .expect("config object must be loaded");
        assert_eq!(&db_url, &config.db_url);
        assert_eq!(&controller_addr, &config.controller_addr);
        assert_eq!(&controller_bind, &config.controller_bind);
        assert_eq!(&cluster_gossip_addr, &config.cluster_gossip_addr);
        assert_eq!(&cluster_gossip_bind, &config.cluster_gossip_bind);
        assert_eq!(&mq_addr, &config.mq_addr);
        assert_eq!(&use_json_log, &config.use_json_log);
        assert_eq!(&log_filter, &config.log_filter);
        testutils::io::remove(&path).expect("temporary confiiguration file should be removed");
    }

    #[test]
    #[serial]
    #[ignore]
    fn test_new_none() {
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
        let config: crate::config::Config = new(None)
            .build()
            .expect("builder should be able to build configuration")
            .try_deserialize()
            .expect("config object must be loaded");
        assert_eq!(&db_url, &config.db_url);
        assert_eq!(&controller_addr, &config.controller_addr);
        assert_eq!(&controller_bind, &config.controller_bind);
        assert_eq!(&cluster_gossip_addr, &config.cluster_gossip_addr);
        assert_eq!(&cluster_gossip_bind, &config.cluster_gossip_bind);
        assert_eq!(&mq_addr, &config.mq_addr);
        assert_eq!(&use_json_log, &config.use_json_log);
        assert_eq!(&log_filter, &config.log_filter);
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
