use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[clap(short, long)]
        address: String,

        #[clap(short, long)]
        port: u16,
    },
}
