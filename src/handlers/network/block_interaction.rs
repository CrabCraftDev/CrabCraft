use crate::components::current_world::CurrentWorld;
use crate::events::player::packet::PacketEvent;
use crab_protocol::packets::play::{PlayerAction, PlayerActionC2SPacket};
use crab_world::blocks::{Air, BlockType};
use evenio::event::Receiver;

pub fn on_block_break(receiver: Receiver<PacketEvent, &CurrentWorld>) {
    let PacketEvent {
        packet,
        protocol_state: state,
    } = receiver.event;

    // TODO: Temporary implementation of block break listening. Will be properly implemented in v0.0.7
    if let Some(packet) = packet.decode::<PlayerActionC2SPacket>(*state) {
        if packet.action == PlayerAction::StartedDigging {
            let player_world = receiver.query.get_world().unwrap();
            player_world
                .set_block(
                    packet.position.x,
                    packet.position.y,
                    packet.position.z,
                    Air::default_state_id().unwrap(),
                )
                .unwrap();
        }
    }
}
