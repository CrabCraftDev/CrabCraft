use crab_protocol::{protocol_state::ProtocolState, raw_packet::RawPacket};
use evenio::{
    entity::EntityId,
    event::{Despawn, Receiver, Sender, TargetedEvent},
    fetch::Fetcher,
};
use log::info;

use crate::{network::connection::NetworkConnection, world::WorldTick};

#[derive(TargetedEvent)]
pub struct PacketEvent {
    pub packet: RawPacket,
    pub protcol_state: ProtocolState,
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
                    let protcol_state = player_connection.get_state();
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
