use hyper::service::{make_service_fn, service_fn};
use hyper::Server as hyperServer;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::mpsc;

use bitfab_utils;

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing Http server at {} on {}", address, port);
        Self {
            address: IpAddr::from_str(&address).unwrap(),
            port,
        }
    }

    pub async fn start(
        &self,
        r: router::Router,
        tx: mpsc::Sender<bitfab_utils::ActorMessage>,
    ) -> Result<(), hyper::Error> {
        let addr: SocketAddr = SocketAddr::new(self.address, self.port);
        //let listener = TcpListener::bind(addr).await.unwrap();
        let r = Arc::new(r);
        // use if hyper::Server is used
        let make_svc = make_service_fn(move |_conn| {
            let r = r.clone();
            let tx = tx.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let r = r.clone();
                    let tx = tx.clone();
                    async move {
                        let mut s = req.method().to_string();
                        s = s + &req.uri().to_string();
                        let a = r.handle(s, req, tx).await;
                        Ok::<_, Infallible>(a)
                    }
                }))
            }
        });

        // When using hyper as internal server
        let server = hyperServer::try_bind(&addr)?.serve(make_svc);
        println!("Listening HTTP on http://{}", addr);
        let _ = server.await?;
        Ok(())
    }
}
