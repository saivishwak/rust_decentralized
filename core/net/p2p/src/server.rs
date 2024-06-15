/*

   Main P2P Server source

*/
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
//use tokio::net::TcpSocket;

use crate::message::{GossipTypes, Message};
//use crate::peer;
use crate::peer::Peer;
use crate::router;
use crate::utils;
use bitfab_utils;

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub peers: Arc<Mutex<Vec<Peer>>>,
    pub router: Arc<router::Router>,
}

impl Server {
    pub async fn broadcast_to_peers(&self, _: Message, stream_id: SocketAddr, broadcast_port: u16) {
        println!("Broadcast initiated");

        let peers = self.peers.lock().await;
        let peers_len = peers.len();

        for index in 0..peers_len {
            if peers[index].stream_id != stream_id {
                let message = Message::new(
                    GossipTypes::ProcessNewPeer,
                    "",
                    Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                    broadcast_port,
                );

                let response = message.marshall();
                let resp = match response {
                    Ok(res) => res,
                    Err(_) => String::new(),
                };

                let mut a = peers[index].socket_stream.lock().await;
                println!("Broadcasting to peer {}", peers[index].port);
                let stream_ready = a
                    .ready(Interest::READABLE | Interest::WRITABLE)
                    .await
                    .unwrap();
                if stream_ready.is_writable() {
                    match a.write_all(resp.as_bytes()).await {
                        Ok(_) => {
                            println!("Successfully sent braodcast message");
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("Error in would block write for broadcast");
                            continue;
                        }
                        Err(e) => {
                            println!("Error sending message for broadcast {}", e);
                        }
                    }
                }
            }
        }
    }
}

// Main Wrapper implementation to contain the mutex
pub struct ServerWrapper {
    pub inner: Arc<Server>,
}

impl ServerWrapper {
    pub fn new(address: String, port: u16, router: Arc<router::Router>) -> Self {
        println!("Initializing the P2P server at {} on {}", address, port);
        let server = Server {
            address: IpAddr::from_str(&address).unwrap(),
            port,
            peers: Arc::new(Mutex::new(Vec::new())),
            router: router.clone(),
        };
        Self {
            inner: Arc::new(server),
        }
    }

    pub async fn start(
        &self,
        mut rx: mpsc::Receiver<bitfab_utils::ActorMessage>,
    ) -> Result<(), std::io::Error> {
        let inner_self = self.inner.clone();
        let server_addr = inner_self.address;
        let server_port = inner_self.port;

        let addr: SocketAddr = SocketAddr::new(server_addr, server_port);

        let listener = TcpListener::bind(addr).await?;
        let server_router = inner_self.router.clone();

        let t1 = tokio::spawn(async move {
            loop {
                let stream = listener.accept().await;
                match stream {
                    Ok(stream_data) => {
                        let stream_data_clone = Arc::new(Mutex::new(stream_data.0));
                        println!("Accepted new connection from {}", stream_data.1.to_string());
                        //let stream_id = Uuid::new_v4();

                        //Add the peer to Peer List
                        let stream_id = stream_data.1;

                        // moved push peer to route handler
                        /*
                        inner_self.peers.lock().await.push(peer::Peer {
                            socket_stream: stream_data_clone.clone(),
                            stream_id,
                            direction: peer::PeerDirection::Inbound,
                            address: Some(stream_data.1.ip()),
                            port: 0,
                        });*/

                        // Send the new connection to handler
                        let stream_data_clone_1 = stream_data_clone.clone();
                        tokio::spawn({
                            let inner_self = inner_self.clone();
                            let router = server_router.clone();
                            async move {
                                utils::handle_connection(
                                    inner_self,
                                    stream_data_clone_1,
                                    router.clone(),
                                    stream_id,
                                    stream_data.1,
                                )
                                .await;
                            }
                        });

                        // Send the first message to the incoming request
                        {
                            let stream_data_clone = stream_data_clone.clone();
                            let m = Message::new(
                                GossipTypes::RequestServerInfo,
                                "handshake_init",
                                Some(server_addr),
                                server_port,
                            );
                            let s = m.marshall();
                            match s {
                                Ok(st) => {
                                    let _ =
                                        stream_data_clone.lock().await.write(st.as_bytes()).await;
                                }
                                Err(e) => {
                                    // Don't do anything just log error
                                    println!("Error in marshalling {}", e);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        //Don't do anything just log the error
                        println!("Error accepting connection {}", err);
                    }
                }
            }
        });

        // spwan task to connect to bootstrap peer
        let inner_self = self.inner.clone();
        let t2 = tokio::spawn(async move {
            println!("Trying to connect to bootstrap peers");
            if let Err(e) = utils::connect_to_peer(inner_self, 3002).await {
                println!("{}", e);
            }
        });

        // Receive the channel messages
        let inner_self = self.inner.clone();
        let t3 = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match message {
                    bitfab_utils::ActorMessage::GetUniqueId { respond_to } => {
                        // The `let _ =` ignores any errors when sending.
                        //
                        // This can happen if the `select!` macro is used
                        // to cancel waiting for the response.
                        println!("Got message from HTTP channel");
                        let peers = inner_self.peers.lock().await;

                        for index in 0..peers.len() {
                            let message = Message::new(
                                GossipTypes::HandleTask,
                                &index.to_string(),
                                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                                peers[index].port,
                            );

                            let response = message.marshall();
                            let resp = match response {
                                Ok(res) => res,
                                Err(_) => String::new(),
                            };

                            let mut a = peers[index].socket_stream.lock().await;
                            println!("Sending Task to peer {}", peers[index].port);
                            let stream_ready = a
                                .ready(Interest::READABLE | Interest::WRITABLE)
                                .await
                                .unwrap();
                            if stream_ready.is_writable() {
                                match a.write_all(resp.as_bytes()).await {
                                    Ok(_) => {
                                        println!("Successfully sent braodcast message");
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                        println!("Error in would block write for broadcast");
                                        continue;
                                    }
                                    Err(e) => {
                                        println!("Error sending message for broadcast {}", e);
                                    }
                                }
                            }
                        }

                        let _ = respond_to.send(12);
                    }
                }
            }
        });

        // Spwan task to log number of peer's connected
        let inner_self = self.inner.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(3000)).await;
                let peers = &inner_self.peers.lock().await;
                println!("Total Number of peers - {}", peers.len());
            }
        });

        // Get the tasks to completion parallely - means on different threads
        let (_, _, _) = tokio::join!(t1, t2, t3);

        Ok(())
    }
}
