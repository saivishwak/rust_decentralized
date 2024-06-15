use clap::Parser;

//Submod files
mod commands;
use super::http_routes;
use super::p2p_routes;
mod types;

#[derive(Parser)]
#[clap(name = "BitFab")]
#[clap(author = "BitFab")]
#[clap(version = "1.0")]
#[clap(about = "Distributed Computing Platform", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<types::Commands>,
}

impl Cli {
    pub async fn init() {
        let cmd: Cli = self::Cli::parse();
        match &cmd.command {
            Some(types::Commands::Start { address, port }) => {
                commands::start(address.clone(), port.clone()).await;
            }
            None => {}
        }
    }
}
