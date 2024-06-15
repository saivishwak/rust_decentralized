use serde;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::from_utf8;

pub enum MessageSuccessStatusCode {
    Success,
    ClosConnection,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GossipTypes {
    Ping,
    Pong,
    RequestServerInfo,
    ProcessServerInfo,
    ProcessNewPeer,
    HandleTask,
    Def,
}

pub trait FromString {
    fn from_string(input: &String) -> Result<GossipTypes, ()>;
}

impl FromString for GossipTypes {
    fn from_string(input: &String) -> Result<GossipTypes, ()> {
        match input.trim() {
            "ping" => Ok(GossipTypes::Ping),
            "pong" => Ok(GossipTypes::Pong),
            "default" => Ok(GossipTypes::Def),
            _ => {
                return Err(());
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    address: Option<IpAddr>,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    pub peer_info: PeerInfo,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub gossip_type: GossipTypes,
    pub body: MessageBody,
}

impl Message {
    pub fn new(gossip_type: GossipTypes, s: &str, address: Option<IpAddr>, port: u16) -> Self {
        Self {
            gossip_type,
            body: MessageBody {
                peer_info: PeerInfo { address, port },
                body: String::from(s),
            },
        }
    }

    pub fn marshall(&self) -> Result<String, serde_json::Error> {
        let this = self;
        let json_marshalled = serde_json::to_string(this)?;
        Ok(json_marshalled)
    }

    pub fn unmarshall(data: &Vec<u8>) -> Result<Message, serde_json::Error> {
        let message: Message = serde_json::from_str(from_utf8(&data).unwrap())?;
        Ok(message)
    }
}
