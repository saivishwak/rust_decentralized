//! Peer to Peer library can be used to instantiate a p2p server and configure routes

pub mod message;
mod peer;
pub mod router;
mod server;
pub mod utils;
pub use message::GossipTypes;
pub use peer::{Peer, PeerDirection};
pub use server::Server;
pub use server::ServerWrapper;
