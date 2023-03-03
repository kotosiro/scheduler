use anyhow::Context;
use anyhow::Result;
use kotosiro::config::Config;
use kotosiro::controller::Controller;
use kotosiro::logging;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let app = clap::Command::new("kotosiro")
        .author("Shingo OKAWA <shingo.okawa.g.h.c@gmail.com>")
        .version(kotosiro::VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("config")
                .long("config")
                .short('c')
                .help("Provide a specific config file"),
        )
        .subcommand(
            clap::Command::new("controller")
                .about("Launch the controller process")
                .after_help("The controller has an API server embedded."),
        )
        .subcommand(clap::Command::new("runner").about("Launch the runner process"));
    let args = app.get_matches();
    let conf = args.get_one::<String>("config").map(AsRef::as_ref);
    let conf = Config::load(conf)?;
    logging::setup(&conf);
    debug!(
        db_url = &conf.db_url,
        controller_addr = &conf.controller_addr,
        controller_bind = &conf.controller_bind,
        cluster_gossip_addr = &conf.cluster_gossip_addr,
        cluster_gossip_bind = &conf.cluster_gossip_bind,
        mq_addr = &conf.mq_addr,
    );
    match args.subcommand().expect("subcommand is required") {
        ("controller", _args) => {
            debug!("controller is called");
            let controller = Controller::new(conf)
                .await
                .context("failed to create controller")?;
            controller
                .start()
                .await
                .context("failed to start controller")?;
            Ok(())
        }
        ("runner", _args) => {
            debug!("runner is called");
            Ok(())
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
