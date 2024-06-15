mod cli;
use cli::Cli;
mod http_routes;
mod p2p_routes;

#[tokio::main]
async fn main() {
    Cli::init().await;
}
