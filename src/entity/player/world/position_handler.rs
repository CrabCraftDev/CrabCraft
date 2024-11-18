use crate::entity::components::name::Name;
use crate::entity::player::{LastTeleport, MoveEvent, TeleportEvent};
use crate::network::connection::NetworkConnection;
use crab_protocol::packets::play::PlayerPositionS2CPacket;
use crab_protocol::types::position::{Position, Vec3Double};
use evenio::entity::EntityId;
use evenio::event::{Receiver, Sender};
use tracing::warn;

pub const MAX_SINGLE_PACKET_MOVE_SQ: f64 = 100f64;

pub fn on_teleport(receiver: Receiver<TeleportEvent, (&NetworkConnection, &mut Position, &mut LastTeleport)>) {
    let (connection, current_pos, last_teleport) = receiver.query;
    let TeleportEvent { position, rotation } = receiver.event;
    current_pos.0 = position.0.clone();

    last_teleport.id += 1;
    last_teleport.confirmed = false;
    connection.send_packet(PlayerPositionS2CPacket {
        position: position.clone(),
        delta_movement: Vec3Double::new(0., 0., 0.),
        rotation: *rotation,
        position_flags: Default::default(),
        teleport_id: last_teleport.id,
    });
}

pub fn on_move(
    receiver: Receiver<MoveEvent, (EntityId, &mut Position, &mut LastTeleport, &Name)>,
    mut teleport: Sender<TeleportEvent>,
) {
    let MoveEvent { position, rotation } = receiver.event;
    let (entity, old_pos, last_teleport, name) = receiver.query;

    if !last_teleport.confirmed {
        return;
    }
    let delta_x = (position.0.x - old_pos.0.x).abs();
    let delta_y = (position.0.y - old_pos.0.y).abs();
    let delta_z = (position.0.z - old_pos.0.z).abs();
    let delta_sq = delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2);

    if delta_sq > MAX_SINGLE_PACKET_MOVE_SQ {
        warn!(
            "Player {} moved too quickly! {}, {}, {}",
            name, delta_x, delta_y, delta_z
        );
        teleport.send_to(
            entity,
            TeleportEvent {
                position: old_pos.clone(),
                rotation: *rotation,
            },
        );
    }

    old_pos.0 = position.0.clone();
}
