use super::{
    handlers::{handshake_handler, keep_alive_handler, login_handler, status_handler},
    keep_alive_state::KeepAliveState,
};
use crate::events::asynchronous::{AsyncEvent, AsyncEventHolder};
use crate::events::player::packet::PacketEvent;
use crate::events::player::player_quit::{PlayerQuitEvent, PlayerQuitReason};
use crate::utils::errors::Error;
use chrono::Local;
use crab_protocol::error::ProtocolError;
use crab_protocol::packet_transformer::PacketTransformer;
use crab_protocol::{
    buf::packet_from_stream,
    packet::Packet,
    packets::{
        common::{DisconnectS2CPacket, KeepAliveResponseC2SPacket, KeepAliveS2CPacket},
        login::LoginDisconnectS2CPacket,
    },
    protocol_state::ProtocolState,
    raw_packet::RawPacket,
};
use crab_text::text::Text;
use crossbeam::sync::ShardedLock;
use evenio::component::Component;
use evenio::entity::EntityId;
use evenio::prelude::{GlobalEvent, TargetedEvent};
use kanal::{AsyncSender, Receiver, Sender};
use parking_lot::Mutex;
use std::fmt::Debug;
use std::net::IpAddr;
use std::{io, panic, sync::Arc, time::Duration};
use tokio::{
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    select,
    time::sleep,
};
use tracing::{error, warn};

const KEEPALIVE_INTERVAL: Duration = Duration::from_millis(15_000);

/// Represents a network connection between player and the server.
/// Used to send packets, receive and manage connection
#[derive(Debug, Clone, Component)]
pub struct NetworkConnection {
    pub ip_address: IpAddr,
    pub entity_id: Arc<ShardedLock<Option<EntityId>>>,
    /// State is accessed from the main thread and connection thread
    pub state: Arc<ShardedLock<ProtocolState>>,
    pub keep_alive_state: Arc<Mutex<KeepAliveState>>,
    pub transformer: PacketTransformer,
    pub outgoing_packets_sender: Sender<RawPacket>,
    pub outgoing_packets_receiver: Receiver<RawPacket>,
    pub event_sender: AsyncSender<AsyncEventHolder>,
    prefix: Arc<ShardedLock<String>>,
}

impl NetworkConnection {
    pub fn new(ip_address: IpAddr, event_sender: AsyncSender<AsyncEventHolder>) -> Self {
        let (outgoing_packets_sender, outgoing_packets_receiver) = kanal::unbounded::<RawPacket>();

        NetworkConnection {
            ip_address,
            entity_id: Arc::new(ShardedLock::default()),
            state: Arc::new(ShardedLock::new(ProtocolState::Handshake)),
            keep_alive_state: Arc::new(Mutex::new(KeepAliveState::default())),
            transformer: PacketTransformer::new(),
            event_sender,
            outgoing_packets_sender,
            outgoing_packets_receiver,
            prefix: Arc::new(format!("{ip_address} connection: ").to_owned().into()),
        }
    }

    pub async fn accept(&self, stream: TcpStream) {
        let (read_half, write_half) = stream.into_split();

        let writer = tokio::spawn(NetworkConnection::writer(
            self.clone(),
            write_half,
            self.transformer.clone(),
        ));

        let reader = tokio::spawn(NetworkConnection::reader(
            self.clone(),
            read_half,
            self.transformer.clone(),
        ));

        let result = select! {
             result = reader => result,
             result = writer => result,
             _ = self.keep_connection_alive() => Ok(()),
        };

        if let Err(err) = result {
            error!("");
            error!("{}thread completed with error: {:?}", self.prefix(), err);
        }

        if !self.is_closed() {
            if self.get_state().should_entity_be_spawned() && self.entity_id.read().unwrap().is_none() {
                error!("Player entity is not spawned!")
            }
            self.close();
        }
        self.send_target_event(PlayerQuitEvent::new(PlayerQuitReason::Disconnected))
            .unwrap();
    }

    #[tracing::instrument(name = "Connection Writer", skip_all)]
    async fn writer(connection: NetworkConnection, mut read_half: OwnedWriteHalf, mut transformer: PacketTransformer) {
        let receiver = connection.outgoing_packets_receiver.clone().to_async();
        loop {
            let Ok(packet) = receiver.recv().await else {
                break;
            };

            let state = *connection.state.read().unwrap();

            match transformer.encode_and_write(packet, state, &mut read_half).await {
                Ok(_) => {}
                Err(err) => {
                    error!("{}Serialization error: {err}", connection.prefix());
                    connection.close();
                    break;
                }
            }
        }
    }

    #[tracing::instrument(name = "Connection Reader", skip_all)]
    async fn reader(connection: NetworkConnection, mut read_half: OwnedReadHalf, mut transformer: PacketTransformer) {
        while !connection.is_closed() {
            let packet = match NetworkConnection::next_packet(&mut read_half, &mut transformer).await {
                Ok(Some(packet)) => packet,
                Ok(None) => break,
                Err(Error::Protocol(ProtocolError::Io(error))) => {
                    let message = match error.kind() {
                        io::ErrorKind::ConnectionReset => "Connection reset".to_owned(),
                        _ => format!("IO Error: {}", error),
                    };
                    connection.disconnect(message.into());
                    break;
                }
                Err(error) => {
                    warn!("{}Error decoding packet {error:?}!", connection.prefix());
                    connection.disconnect("Invalid packet received!".into());
                    break;
                }
            };

            let process_packet_result = connection.process_packet(packet).await;
            if let Err(error) = process_packet_result {
                panic!(
                    "{}Unexpected error while handling packet {error:?}!",
                    connection.prefix()
                );
            }
        }
    }

    pub async fn keep_connection_alive(&self) {
        loop {
            sleep(KEEPALIVE_INTERVAL).await;
            let state = self.get_state();
            if state != ProtocolState::Configuration && state != ProtocolState::Play {
                continue;
            }

            let current_time;
            {
                let mut keep_alive_state = self.keep_alive_state.lock();

                if keep_alive_state.keep_alive_pending {
                    self.disconnect("Timeout!".into());
                    continue;
                }

                current_time = Local::now().timestamp_millis();
                keep_alive_state.keep_alive_pending = true;
                keep_alive_state.last_keep_alive_time = current_time;
            }

            self.send_packet(KeepAliveS2CPacket {
                keep_alive_id: current_time,
            });
        }
    }

    async fn next_packet(
        read_half: &mut OwnedReadHalf,
        transformer: &mut PacketTransformer,
    ) -> Result<Option<RawPacket>, Error> {
        Ok(packet_from_stream(read_half)
            .await?
            .map(|buffer| transformer.decode(buffer))
            .transpose()?)
    }

    pub async fn process_packet(&self, raw_packet: RawPacket) -> Result<(), Error> {
        let state = self.get_state();
        match state {
            ProtocolState::Status => status_handler::handle_packet(self, raw_packet).await,
            ProtocolState::Handshake => handshake_handler::handle_packet(self, raw_packet),
            ProtocolState::Login => login_handler::handle_packet(self, raw_packet).await?,
            ProtocolState::Configuration | ProtocolState::Play => {
                if let Some(keep_alive_packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>(state) {
                    keep_alive_handler::handle_packet(self, keep_alive_packet).await;
                } else {
                    self.send_target_event(PacketEvent {
                        packet: raw_packet,
                        protocol_state: self.get_state(),
                    })?;
                }
            }
        };
        Ok(())
    }

    pub fn send_packet<P: Packet>(&self, packet: P) {
        match RawPacket::try_from_packet(packet, self.get_state()) {
            Ok(raw_packet) => {
                if self.outgoing_packets_sender.send(raw_packet).is_err() {
                    warn!("{}Tried to send packet while connection closed", self.prefix());
                }
            }
            Err(err) => {
                error!("{}Serialization error: {err}", self.prefix());
            }
        };
    }

    pub fn disconnect(&self, message: Text) {
        self.disconnect_with_reason(message.clone(), PlayerQuitReason::Kicked(message.into()));
    }

    pub fn disconnect_with_reason(&self, message: Text, reason_type: PlayerQuitReason) {
        if self.is_closed() {
            warn!("Called disconnect() on a closed connection");
            return;
        }

        // Don't send event if there is no entity spawned in ECS
        if self.entity_id.read().unwrap().is_some() {
            self.send_target_event(PlayerQuitEvent::new(reason_type)).unwrap();
        } else if self.get_state().should_entity_be_spawned() {
            panic!("Entity is not spawned in {:?} protocol state", self.get_state());
        }

        match self.get_state() {
            ProtocolState::Configuration | ProtocolState::Play => {
                self.send_packet(DisconnectS2CPacket { reason: message });
            }
            ProtocolState::Login => {
                self.send_packet(LoginDisconnectS2CPacket { reason: message });
            }
            _ => {}
        }

        self.close();
    }

    pub fn is_closed(&self) -> bool {
        self.outgoing_packets_sender.is_closed()
    }

    pub fn close(&self) {
        self.outgoing_packets_sender.close();
    }

    pub fn get_state(&self) -> ProtocolState {
        *self.state.read().unwrap()
    }

    pub fn set_state(&self, new_state: ProtocolState) {
        *self.state.write().unwrap() = new_state;
    }

    pub fn prefix(&self) -> String {
        self.prefix.read().unwrap().clone()
    }

    pub fn set_prefix(&self, player_name: &str) {
        *self.prefix.write().unwrap() = format!("{player_name}'s connection: ")
    }

    #[allow(unused)]
    pub fn send_event<E: GlobalEvent + TryFrom<AsyncEvent> + Into<AsyncEvent>>(&self, event: E) -> Result<(), Error> {
        debug_assert!(!self.is_closed(), "Tried to send event on closed connection");

        AsyncEvent::send_event(event, &self.event_sender)
    }

    pub fn send_target_event<E: TargetedEvent + Debug + TryFrom<AsyncEvent> + Into<AsyncEvent>>(
        &self,
        event: E,
    ) -> Result<(), Error> {
        if self.entity_id.read().unwrap().is_none() {
            return Ok(());
        }

        let id = *self.entity_id.clone().read().unwrap();
        AsyncEvent::send_target_event(event, &self.event_sender, id)
    }

    pub async fn send_event_with_response<E: GlobalEvent + Debug + TryFrom<AsyncEvent> + Into<AsyncEvent>>(
        &self,
        event: E,
    ) -> Result<E, Error> {
        debug_assert!(
            !self.is_closed(),
            "Tried to send event with response on closed connection"
        );

        AsyncEvent::send_event_with_response(event, &self.event_sender).await
    }
}
