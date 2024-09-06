use crate::network::connection::Connection;
use crate::network::packet_transformer::COMPRESSION_THRESHOLD;
use crab_protocol::packets::login::{
    LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket, SetCompressionS2CPacket,
};
use crab_protocol::packets::status::PlayerSample;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use evenio::entity::EntityId;
use evenio::event::Receiver;
use evenio::prelude::{Fetcher, Insert, Sender};

pub fn login_packets(
    receiver: Receiver<RawPacket, (EntityId, &mut Connection)>,
    fetcher: Fetcher<&PlayerSample>,
    mut sender: Sender<Insert<PlayerSample>>,
) {
    let (entity, connection) = receiver.query;

    let packet = receiver.event;
    let state = connection.state();
    if state != ProtocolState::Login {
        return;
    }

    if let Some(packet) = packet.decode::<LoginStartC2SPacket>(state) {
        connection.send_packet(SetCompressionS2CPacket {
            threshold: COMPRESSION_THRESHOLD,
        });

        if fetcher
            .iter()
            .any(|sample| sample.name == packet.username || sample.id == packet.player_uuid)
        {
            connection.disconnect("You logged in from another location!");
            return;
        }

        let sample = PlayerSample {
            name: packet.username.clone(),
            id: packet.player_uuid,
        };
        sender.insert(entity, sample);
        connection.send_packet(LoginSuccessS2CPacket {
            uuid: packet.player_uuid,
            username: packet.username,
            properties: Vec::new(),
            strict_protocol_error_handling: true,
        });
    }

    if let Some(_packet) = packet.decode::<LoginAcknowledgedC2SPacket>(state) {
        connection.set_state(ProtocolState::Configuration);
    }
}
