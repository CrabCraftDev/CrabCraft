use crate::components::name::Name;
use crate::components::player::LastTeleport;
use crate::events::player::movement::{PlayerMoveEvent, PlayerTeleportEvent};
use crate::events::player::packet::PacketEvent;
use crate::network::connection::NetworkConnection;
use crate::world::chunk_sender::send_chunks_on_movement;
use crab_protocol::packets::play::{
    ConfirmTeleportationC2SPacket, PlayerPositionS2CPacket, SetPlayerPositionAndRotationC2SPacket,
    SetPlayerPositionC2SPacket, SetPlayerRotationC2SPacket,
};
use crab_protocol::types::player_movement_flags::PlayerMovementFlags;
use crab_protocol::types::position::{Position, Rotation, Vec3Double};
use evenio::entity::EntityId;
use evenio::event::{Insert, Receiver, Sender};
use tracing::warn;

pub const MAX_SINGLE_PACKET_MOVE_SQ: f64 = 100.0;

pub fn on_teleport(receiver: Receiver<PlayerTeleportEvent, (&NetworkConnection, &mut Position, &mut LastTeleport)>) {
    let (connection, current_pos, last_teleport) = receiver.query;
    let PlayerTeleportEvent { position, rotation } = receiver.event;
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
    receiver: Receiver<PlayerMoveEvent, (EntityId, &mut Position, &mut LastTeleport, &Name, &NetworkConnection)>,
    teleport: Sender<PlayerTeleportEvent>,
) {
    let PlayerMoveEvent { position, rotation } = receiver.event;
    let (entity, old_pos, last_teleport, name, connection) = receiver.query;

    if !last_teleport.confirmed {
        return;
    }
    let delta_x = position.0.x - old_pos.0.x;
    let delta_y = position.0.y - old_pos.0.y;
    let delta_z = position.0.z - old_pos.0.z;
    let delta_sq = delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2);

    if delta_sq > MAX_SINGLE_PACKET_MOVE_SQ {
        warn!(
            "Player {} moved too quickly! {}, {}, {}",
            name, delta_x, delta_y, delta_z
        );
        teleport.send_to(
            entity,
            PlayerTeleportEvent {
                position: old_pos.clone(),
                rotation: *rotation,
            },
        );
    }

    send_chunks_on_movement(connection, old_pos, position);
    old_pos.0 = position.0.clone();
}

pub fn move_packet_receiver(
    receiver: Receiver<PacketEvent, (&mut Rotation, &mut LastTeleport, EntityId)>,
    sender: Sender<(PlayerMoveEvent, Insert<PlayerMovementFlags>)>,
) {
    let (rot, last_teleport, entity) = receiver.query;

    let state = receiver.event.protocol_state;
    let packet = &receiver.event.packet;

    if let Some(confirm_teleport) = packet.decode::<ConfirmTeleportationC2SPacket>(state) {
        if last_teleport.id == confirm_teleport.teleport_id {
            last_teleport.confirmed = true;
        }
    }

    if let Some(packet) = packet.decode::<SetPlayerPositionC2SPacket>(state) {
        sender.send_to(
            entity,
            PlayerMoveEvent {
                position: packet.position,
                rotation: *rot,
            },
        );
        sender.insert(entity, packet.movement_flags)
    };

    if let Some(packet) = packet.decode::<SetPlayerPositionAndRotationC2SPacket>(state) {
        sender.send_to(
            entity,
            PlayerMoveEvent {
                position: packet.position,
                rotation: packet.rotation,
            },
        );
        rot.yaw = packet.rotation.yaw;
        rot.pitch = packet.rotation.pitch;
        sender.insert(entity, packet.movement_flags)
    }

    if let Some(packet) = packet.decode::<SetPlayerRotationC2SPacket>(state) {
        rot.yaw = packet.rotation.yaw;
        rot.pitch = packet.rotation.pitch;
        sender.insert(entity, packet.movement_flags)
    }
}
