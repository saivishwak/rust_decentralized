use tokio::sync::oneshot;

pub enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}
