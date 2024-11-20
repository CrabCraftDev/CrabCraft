use crate::events::global::incoming_connection::IncomingConnection;
use crate::events::global::server_list_ping::ServerListPingEvent;
use crate::events::global::world::WorldTickEvent;
use crate::events::player::packet::PacketEvent;
use crate::events::player::player_pre_login::PlayerPreLoginEvent;
use crate::events::player::player_quit::PlayerQuitEvent;
use crate::utils::errors::Error;
use derive_more::{From, Into};
use evenio::event::{GlobalEvent, TargetedEvent};
use evenio::prelude::EntityId;
use kanal::{AsyncSender, OneshotSender};

#[derive(Debug, From)]
pub enum AsyncEvent {
    WorldTick(WorldTickEvent),
    IncomingConnection(IncomingConnection),
    ServerListPing(ServerListPingEvent),
    PlayerPreLogin(PlayerPreLoginEvent),
    PlayerQuit(PlayerQuitEvent),
    Packet(PacketEvent),
}

pub struct AsyncEventHolder {
    pub event: AsyncEvent,
    pub entity_id: Option<EntityId>,
    pub response_channel: Option<OneshotSender<AsyncEvent>>,
}

impl AsyncEvent {
    /// Sends event without waiting for it to be parsed
    pub fn send_event<E: GlobalEvent + TryFrom<AsyncEvent> + Into<AsyncEvent>>(
        event: E,
        sender: &AsyncSender<AsyncEventHolder>,
    ) -> Result<(), Error> {
        let async_event: AsyncEvent = event.into();

        sender.as_sync().send(AsyncEventHolder {
            event: async_event,
            response_channel: None,
            entity_id: None,
        })?;

        Ok(())
    }

    pub fn send_target_event<E: TargetedEvent + TryFrom<AsyncEvent> + Into<AsyncEvent>>(
        event: E,
        sender: &AsyncSender<AsyncEventHolder>,
        entity_id: Option<EntityId>,
    ) -> Result<(), Error> {
        let async_event: AsyncEvent = event.into();

        sender.as_sync().send(AsyncEventHolder {
            event: async_event,
            response_channel: None,
            entity_id,
        })?;

        Ok(())
    }

    pub async fn send_event_with_response<E: GlobalEvent + TryFrom<AsyncEvent> + Into<AsyncEvent>>(
        event: E,
        sender: &AsyncSender<AsyncEventHolder>,
    ) -> Result<E, Error> {
        let async_event: AsyncEvent = event.into();
        let (async_sender, async_receiver) = kanal::oneshot::<AsyncEvent>();

        sender
            .send(AsyncEventHolder {
                event: async_event,
                response_channel: Some(async_sender),
                entity_id: None,
            })
            .await?;

        let res = async_receiver.to_async().recv().await.map_err(Error::OneshotReceive)?;

        E::try_from(res).map_err(|_| Error::InvalidAsyncEventType)
    }
}
