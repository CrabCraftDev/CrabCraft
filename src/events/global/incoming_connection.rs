use crate::events::asynchronous::AsyncEvent;
use crate::network::connection::NetworkConnection;
use crate::utils::errors::Error;
use evenio::event::GlobalEvent;
use uuid::Uuid as ActualUuid;

#[derive(Debug, Clone, GlobalEvent)]
pub struct IncomingConnection {
    pub connection: NetworkConnection,
    pub player_name: String,
    pub player_uuid: ActualUuid,
}

impl TryFrom<AsyncEvent> for IncomingConnection {
    type Error = Error;

    fn try_from(value: AsyncEvent) -> Result<Self, Self::Error> {
        match value {
            AsyncEvent::IncomingConnection(event) => Ok(event),
            _ => Err(Error::InvalidAsyncEventType),
        }
    }
}
