use anyhow::Result;

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

    match args.subcommand().expect("subcommand is required") {
        ("scheduler", _args) => {
            println!("placeholder");
            Ok(())
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }
}
