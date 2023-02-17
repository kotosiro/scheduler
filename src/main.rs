use anyhow::Result;
use kotosiro::config;
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
                .about("Launch the scheduler process")
                .after_help("The scheduler has an API server embedded."),
        )
        .subcommand(
            clap::Command::new("api")
                .about("Launch the API server process")
                .after_help("The API server may be launched many times for load balancing and HA."),
        );

    let args = app.get_matches();
    let conf = args.get_one::<String>("config").map(AsRef::as_ref);
    let conf = config::load(conf)?;
    logging::setup(&conf)?;
    debug!("configuration: {:?}", &conf);

    match args.subcommand().expect("subcommand is required") {
        ("controller", _args) => {
            println!("controller");
            Ok(())
        }
        ("api", _args) => {
            println!("api");
            Ok(())
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
