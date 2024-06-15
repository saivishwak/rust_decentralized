use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
//use uuid::Uuid;
use std::net::SocketAddr;

use p2p;
use p2p::message::Message;
use p2p::GossipTypes;
use p2p::{Peer, PeerDirection};

// function to configure p2p router
pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |message: Message,
         _: SocketAddr,
         server_state: Arc<p2p::Server>,
         _: Arc<Mutex<tokio::net::TcpStream>>,
         _: SocketAddr| async move {
            let server_addr = server_state.address;
            let server_port = server_state.port;
            println!("Ping Handler - Server Add {} {:?}", server_addr, message);
            //To simluate async
            sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Pong, "Ponging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |message: Message,
         _: SocketAddr,
         server_state: Arc<p2p::Server>,
         _: Arc<Mutex<tokio::net::TcpStream>>,
         _: SocketAddr| async move {
            let server_addr = server_state.address;
            let server_port = server_state.port;

            println!("Pong Handler - Server Add {} {:?}", server_addr, message);
            //sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Ping, "Pinging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::RequestServerInfo,
        |_: Message,
         _: SocketAddr,
         server_info: Arc<p2p::Server>,
         _: Arc<Mutex<tokio::net::TcpStream>>,
         _: SocketAddr| async move {
            println!("Request server info handler");
            let server_addr = server_info.address;
            let server_port = server_info.port;

            let message = Message::new(
                GossipTypes::ProcessServerInfo,
                "",
                Some(server_addr),
                server_port,
            );
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::ProcessServerInfo,
        |message: Message,
         stream_id: SocketAddr,
         server_info: Arc<p2p::Server>,
         stream: Arc<Mutex<tokio::net::TcpStream>>,
         remote_addr: SocketAddr| async move {
            println!("Process server info handler");
            let server_addr = server_info.address;
            let server_port = server_info.port;
            let new_peer_port = message.body.peer_info.port;

            let message = Message::new(
                GossipTypes::ProcessNewPeer,
                "",
                Some(server_addr),
                server_port,
            );
            let _ = server_info
                .broadcast_to_peers(message, stream_id, new_peer_port)
                .await;

            server_info.peers.lock().await.push(Peer {
                socket_stream: stream.clone(),
                stream_id,
                direction: PeerDirection::Inbound,
                address: Some(remote_addr.ip()),
                port: new_peer_port,
            });

            None
        },
    );

    router.add_handler(
        GossipTypes::ProcessNewPeer,
        |message: Message,
         _: SocketAddr,
         server_info: Arc<p2p::Server>,
         _: Arc<Mutex<tokio::net::TcpStream>>,
         _: SocketAddr| async move {
            println!("Process New Peer handler");
            let server_port = server_info.port;
            let mut found = false;
            let p = message.body.peer_info.port;

            {
                let peers = &server_info.peers.lock().await;
                for index in 0..peers.len() {
                    if peers[index].port == p {
                        found = true;
                    }
                }
            }

            if !found {
                if server_port != p {
                    let _ = tokio::task::spawn(async move {
                        if let Err(e) = p2p::utils::connect_to_peer(server_info, p).await {
                            println!("{}", e);
                        }
                    });
                    //let _ = tokio::join!(a);
                }
            } else {
                println!("Found Peer - ingoring connect to peer");
            }

            None
        },
    );

    router.add_handler(
        GossipTypes::HandleTask,
        |message: Message,
         _: SocketAddr,
         _: Arc<p2p::Server>,
         _: Arc<Mutex<tokio::net::TcpStream>>,
         _: SocketAddr| async move {
            println!("Handle Task handler {}", message.body.body);
            println!("Handling Task with Data {}", message.body.body);
            for index in 0..10 {
                println!("{}", index);
            }
            None
        },
    );
}
