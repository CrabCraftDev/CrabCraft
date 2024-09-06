use crate::entity::player::{KeepAliveStatus, PlayerDisconnected};
use crate::network::connection::Connection;
use crate::world::{Shutdown, WorldTick};
use chrono::Local;
use crab_protocol::packets::common::{KeepAliveResponseC2SPacket, KeepAliveS2CPacket};
use crab_protocol::packets::status::PlayerSample;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use evenio::{
    entity::EntityId,
    event::{Despawn, Receiver, Sender},
    fetch::Fetcher,
};
use log::info;

const KEEPALIVE_INTERVAL_MS: i64 = 15_000;

pub fn receive_packets(
    _receiver: Receiver<WorldTick>,
    mut event_sender: Sender<(RawPacket, PlayerDisconnected)>,
    players: Fetcher<(EntityId, &Connection)>,
) {
    for (entity, connection) in players {
        loop {
            if connection.is_closed() {
                event_sender.send_to(entity, PlayerDisconnected);
                break;
            }

            match connection.incoming_packets_receiver.try_recv() {
                Ok(Some(packet)) => event_sender.send_to(entity, packet),
                Ok(None) => break,
                Err(_) => {}
            };
        }
    }
}

pub fn kick_on_shutdown(_: Receiver<Shutdown>, players: Fetcher<&mut Connection>) {
    for connection in players {
        connection.disconnect("Server shutting down!");
    }
}

pub fn on_disconnect(receiver: Receiver<PlayerDisconnected, (EntityId, &PlayerSample)>, mut sender: Sender<Despawn>) {
    let (entity, info) = receiver.query;
    info!("Player {} disconnected!", info.name);
    sender.send_to(entity, Despawn);
}

pub fn keep_alive_tick(_: Receiver<WorldTick>, players: Fetcher<(EntityId, &mut Connection, &mut KeepAliveStatus)>) {
    for (_id, connection, player) in players {
        let state = connection.state();
        if state != ProtocolState::Configuration && state != ProtocolState::Play {
            return;
        }

        let current_time = Local::now().timestamp_millis();
        if current_time - player.last_keepalive_time < KEEPALIVE_INTERVAL_MS {
            return;
        }

        if player.keepalive_pending {
            connection.disconnect("Timeout!");
            return;
        }

        player.keepalive_pending = true;
        player.last_keepalive_time = current_time;
        connection.send_packet(KeepAliveS2CPacket {
            keep_alive_id: current_time,
        });
    }
}

pub fn on_keep_alive_response(receiver: Receiver<RawPacket, (&mut Connection, &mut KeepAliveStatus)>) {
    let (connection, player) = receiver.query;
    if let Some(packet) = receiver.event.decode::<KeepAliveResponseC2SPacket>(connection.state()) {
        if player.keepalive_pending && player.last_keepalive_time == packet.keep_alive_id {
            player.keepalive_pending = false
        } else {
            connection.disconnect("Timeout!");
        };
    }
}
