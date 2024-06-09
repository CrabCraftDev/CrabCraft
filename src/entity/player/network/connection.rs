use std::sync::Arc;

use crab_protocol::{packet::Packet, protocol::ProtocolState, raw_packet::RawPacket};
use crossbeam::sync::ShardedLock;
use evenio::{
    component::Component,
    entity::EntityId,
    event::{Despawn, Receiver, Sender, TargetedEvent},
    fetch::Fetcher,
};
use log::{error, info};

use crate::world::WorldTick;

#[derive(Component)]
pub struct PlayerConnection {
    pub protocol_state: Arc<ShardedLock<ProtocolState>>,
    pub packet_receiver: kanal::Receiver<RawPacket>,
    pub packet_sender: kanal::Sender<RawPacket>,
}

impl PlayerConnection {
    pub fn send_packet<P: Packet>(&self, packet: P) {
        match RawPacket::from_packet(packet, *self.protocol_state.read().unwrap()) {
            Ok(raw_packet) => self
                .packet_sender
                .send(raw_packet)
                .expect("Packet to be sent to network thread"),
            Err(err) => {
                error!("Serialization error: {}", err);
            }
        };
    }
}

#[derive(TargetedEvent)]
pub struct PacketEvent {
    pub packet: RawPacket,
    pub protcol_state: ProtocolState,
}

pub fn receive_packets(
    _: Receiver<WorldTick>,
    mut event_sender: Sender<(PacketEvent, PlayerDisconnectedEvent)>,
    players: Fetcher<(EntityId, &PlayerConnection)>,
) {
    for (entity, player_connection) in players {
        loop {
            match player_connection.packet_receiver.try_recv() {
                Ok(Some(packet)) => {
                    let protcol_state = *player_connection.protocol_state.read().unwrap();
                    event_sender.send_to(entity, PacketEvent { packet, protcol_state });
                }
                Ok(None) => break,
                Err(_) => {
                    event_sender.send_to(entity, PlayerDisconnectedEvent);
                    break;
                }
            };
        }
    }
}

#[derive(TargetedEvent)]
pub struct PlayerDisconnectedEvent;

pub fn on_disconnect(receiver: Receiver<PlayerDisconnectedEvent, EntityId>, mut sender: Sender<Despawn>) {
    let entity = receiver.query;

    info!("Player disconnected!");
    sender.send_to(entity, Despawn);
}
