use hyper::{Body, Response};
use std::path::PathBuf;
use tokio::fs::read_to_string;
use tokio::sync::{mpsc, oneshot};

use bitfab_utils;
use router::{Methods, Router};

// function to configure the routes to router
pub fn configure(router: &mut Router) {
    router.add_handler(
        String::from(Methods::GET.to_string() + "/"),
        |_, tx: mpsc::Sender<bitfab_utils::ActorMessage>| async move {
            let contents = read_to_string(PathBuf::from("./static/hello.html")).await;
            let (send, recv) = oneshot::channel();
            let msg = bitfab_utils::ActorMessage::GetUniqueId { respond_to: send };
            let _ = tx.send(msg).await;
            let a = recv.await.expect("Actor task has been killed");
            println!("Receive {} from P2P Channel", a);
            Response::new(Body::from(contents.unwrap()))
        },
    );

    router.add_handler(
        String::from(Methods::GET.to_string() + "/task"),
        |_, tx: mpsc::Sender<bitfab_utils::ActorMessage>| async move {
            // Submit task
            let (send, recv) = oneshot::channel();
            let msg = bitfab_utils::ActorMessage::GetUniqueId { respond_to: send };
            let _ = tx.send(msg).await;
            let a = recv.await.expect("Actor task has been killed");

            Response::new(Body::from(format!("Task submitted with ID {}", a)))
        },
    );
}
