use crab_protocol::{protocol_state::ProtocolState, raw_packet::RawPacket};
use evenio::{
    entity::EntityId,
    event::{Despawn, Receiver, Sender, TargetedEvent},
    fetch::Fetcher,
};
use log::info;

use crate::{entity::components::name::Name, network::connection::NetworkConnection, world::WorldTick};

#[derive(TargetedEvent)]
pub struct PacketEvent {
    pub packet: RawPacket,
    pub protocol_state: ProtocolState,
}

pub fn receive_packets(
    _: Receiver<WorldTick>,
    mut event_sender: Sender<(PacketEvent, PlayerDisconnectedEvent)>,
    players: Fetcher<(EntityId, &NetworkConnection)>,
) {
    for (entity, player_connection) in players {
        loop {
            match player_connection.incoming_packets_receiver.try_recv() {
                Ok(Some(packet)) => {
                    let protocol_state = player_connection.get_state();
                    event_sender.send_to(entity, PacketEvent { packet, protocol_state });
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

pub fn on_disconnect(receiver: Receiver<PlayerDisconnectedEvent, (EntityId, &Name)>, mut sender: Sender<Despawn>) {
    let (entity, name) = receiver.query;

    info!("{name} disconnected!");
    sender.send_to(entity, Despawn);
}
