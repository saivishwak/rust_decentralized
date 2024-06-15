use crate::message::Message;
use crate::peer;
use crate::router;
use crate::Server;
use std::io;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn handle_connection(
    inner_self: Arc<Server>,
    stream: Arc<Mutex<tokio::net::TcpStream>>,
    router: Arc<router::Router>,
    stream_id: SocketAddr,
    remote_addr: SocketAddr,
) {
    loop {
        let mut stream_mutex_guard = stream.lock().await;
        let mut buffer = Vec::with_capacity(4096);
        let stream_ready = stream_mutex_guard
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await
            .unwrap();
        if stream_ready.is_readable() {
            let stream_data = stream_mutex_guard.try_read_buf(&mut buffer);
            match stream_data {
                Ok(data) => {
                    if data == 0 {
                        println!("Socket Connection disconnected");
                        break;
                    }
                    println!(
                        "Received msg from {} - {:?}",
                        remote_addr.to_string(),
                        str::from_utf8(&buffer)
                    );
                    let gossip_type_res = Message::unmarshall(&buffer);
                    match gossip_type_res {
                        Ok(message) => {
                            let response = router
                                .handle(
                                    message,
                                    stream_id,
                                    inner_self.clone(),
                                    stream.clone(),
                                    remote_addr,
                                )
                                .await;
                            match response {
                                Some(res_string) => {
                                    if stream_ready.is_writable() {
                                        match stream_mutex_guard
                                            .write_all(res_string.as_bytes())
                                            .await
                                        {
                                            Ok(_) => {
                                                println!("Message sent");
                                            }
                                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                                println!("Error in would block write");
                                                continue;
                                            }
                                            Err(e) => {
                                                println!("Error sending message {}", e);
                                            }
                                        }
                                    }
                                }
                                None => {
                                    continue;
                                }
                            }
                        }
                        Err(_) => {
                            println!("Error in decoding type");
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(_) => {
                    println!("Error reading message");
                    //break;
                }
            }
        }
    }
    // Socket must be disconnected clean up the peers vec
    let peers = &mut inner_self.peers.lock().await;
    let mut index = 0;
    let peers_len = peers.len();
    for i in 0..peers_len {
        if peers[i].stream_id == stream_id {
            peers.remove(index);
            break;
        } else {
            index += 1;
        }
    }
}

pub async fn connect_to_peer(server: Arc<Server>, port: u16) -> Result<(), Error> {
    println!("Attempting to connect Peer at {}", port);
    let inner_self = server.clone();
    let router_arc = inner_self.router.clone();

    if inner_self.port != port {
        let tcp_address = SocketAddr::from(([127, 0, 0, 1], port));
        let stream = TcpStream::connect(tcp_address).await?;

        let remote_addr = stream.peer_addr()?;
        let stream_data_clone = Arc::new(Mutex::new(stream));

        println!("Successfully connected to peer at port {}", port);
        //let stream_id = Uuid::new_v4();
        let inner_self = inner_self;
        //let router_arc = router_arc.clone();
        inner_self.peers.lock().await.push(peer::Peer {
            socket_stream: stream_data_clone.clone(),
            stream_id: remote_addr,
            direction: peer::PeerDirection::Outbound,
            address: None,
            port,
        });
        handle_connection(
            inner_self,
            stream_data_clone,
            router_arc.clone(),
            remote_addr,
            remote_addr,
        )
        .await;

        //Return Ok
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Connecting to self ignoring"))
    }
}
