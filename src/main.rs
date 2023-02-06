use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let app = clap::Command::new("datadact")
        .author("Shingo OKAWA <shingo.okawa.g.h.c@gmail.com>")
        .version(datadact::VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("config")
                .long("config")
                .short('c')
                .help("Provide a specific config file"),
        )
        .subcommand(
            clap::Command::new("scheduler")
                .alias("server")
                .about("Launch the scheduler process")
                .after_help("The scheduler has an API server embedded."),
        );

    let args = app.get_matches();

    match args.subcommand().expect("subcommand is required") {
        ("scheduler", _args) => {
            println!("placeholder");
        }
        _ => unreachable!("clap should have already checked the subcommands"),
    }

    Ok(()) // Placeholder
}
