# Data types used in the project

```rust
// "server.rs"
pub struct Server {
    pubaddress: IpAddr,
    pubport: u16,
}

// "types.rs"
pub enum ServerError {
    StartError,
}

// "message.rs"
pub enum MessageSuccessStatusCode {
    Success,
    ClosConnection,
}
// "message.rs"
pub enum GossipTypes {
    Ping,
    Pong,
    RequestServerInfo,
    ProcessServerInfo,
    ProcessNewPeer,
    Def,
}
// "message.rs"
pub struct PeerInfo {
    address: Option<IpAddr>,
    pubport: u16,
}
// "message.rs"
pub struct MessageBody {
    pubpeer_info: PeerInfo,
    body: String,
}
// "message.rs"
pub struct Message {
    pubgossip_type: GossipTypes,
    pubbody: MessageBody,
}

// "peer.rs"
pub enum PeerDirection {
    Inbound,
    Outbound,
}
// "peer.rs"
pub struct Peer {
    pubsocket_stream: Arc<Mutex<TcpStream>>,
    pubstream_id: SocketAddr,
    pubdirection: PeerDirection,
    pubaddress: Option<IpAddr>,
    pubport: u16,
}

// "router.rs"
pub struct Router {
    pubhandlers: HashMap<message::GossipTypes, Option<BoxedRouteHandler>>,
}

// "server.rs"
pub struct Server {
    pubaddress: IpAddr,
    pubport: u16,
    pubpeers: Vec<Peer>,
    pubrouter: Arc<router::Router>,
}
// "server.rs"
pub struct ServerWrapper {
    pubinner: Arc<Mutex<Server>>,
}

// "router.rs"
pub struct Router {
    pubhandlers: HashMap<String, Option<BoxedRouteHandler>>,
}

// "types.rs"
pub enum Methods {}

// "mod.rs"
pub struct Cli {
    #[clap(subcommand)]
    pubcommand: Option<types::Commands>,
}

// "types.rs"
pub enum Commands {
    Start {
        #[clap(short, long)]
        address: String,
        #[clap(short, long)]
        port: u16,
    },
}
```
