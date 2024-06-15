use std::sync::Arc;
use tokio::sync::mpsc;

use super::http_routes;
use super::p2p_routes;
use http_core::Server;
use router::Router;

use bitfab_utils;

pub async fn start(address: String, port: u16) {
    // Initiate http router
    let mut http_router = Router::new();
    http_routes::configure(&mut http_router);

    //Initiate p2p_routers
    let mut p2p_router = p2p::router::Router::new();
    p2p_routes::configure(&mut p2p_router);
    let p2p_router_arc = Arc::new(p2p_router);

    //Initiate p2p_server
    let addr = Arc::new(address);
    let p2p_server: Arc<p2p::ServerWrapper> = Arc::new(p2p::ServerWrapper::new(
        addr.clone().to_string(),
        port + 1,
        p2p_router_arc.clone(),
    ));

    //Create a communication chhanel for http and p2p
    let (tx, rx) = mpsc::channel::<bitfab_utils::ActorMessage>(32);

    // spwan http and p2p tasks and wait for completion
    let (_, _) = tokio::join!(
        tokio::task::spawn({
            let addr = addr.clone();
            async move {
                let http_server: Server = Server::new(addr.to_string(), port);
                if let Err(e) = http_server.start(http_router, tx).await {
                    println!("Error Starting HTTP Server - {}", e);
                }
            }
        }),
        tokio::task::spawn({
            let p2p_server = p2p_server.clone();
            async move {
                let mut p2p_router = p2p::router::Router::new();
                p2p_routes::configure(&mut p2p_router);
                if let Err(e) = p2p_server.start(rx).await {
                    println!("Error starting P2P Server {}", e);
                }
            }
        }),
    );
}
