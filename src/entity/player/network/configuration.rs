use bytes::{Bytes, BytesMut};
use crab_protocol::packets::common::{
    ClientInformationC2SPacket, KnownServerLinkType, PluginMessageS2CPacket, ReportDetail, ReportDetailsS2CPacket,
    ServerLink, ServerLinksS2CPacket,
};
use crab_protocol::{
    packet::Packet,
    packets::configuration::{
        AcknowledgeFinishConfigurationC2SPacket, FinishConfigurationS2CPacket, KnownPacksC2SPacket,
        KnownPacksS2CPacket, RegistryDataS2CPacket,
    },
    protocol_serializer::ProtocolSerializer,
    protocol_state::ProtocolState,
    raw_packet::RawPacket,
    types::{identifier::Identifier, known_pack::KnownPack},
};
use evenio::{
    entity::EntityId,
    event::{Receiver, Sender},
};

pub const REGISTRY_DIMENSION_TYPE_PACKET: &[u8] =
    include_bytes!("../../../../assets/packets/registry_dimension_type.bin");
pub const REGISTRY_DAMAGE_TYPE_PACKET: &[u8] = include_bytes!("../../../../assets/packets/registry_damage_type.bin");
pub const REGISTRY_BIOMES_PACKET: &[u8] = include_bytes!("../../../../assets/packets/registry_biomes.bin");
pub const REGISTRY_PAINTING_VARIANT_PACKET: &[u8] =
    include_bytes!("../../../../assets/packets/registry_painting_variant.bin");
pub const REGISTRY_WOLF_VARIANT_PACKET: &[u8] = include_bytes!("../../../../assets/packets/registry_wolf_variant.bin");

use crate::mc_server::SERVER_VERSION;

use super::{
    connection::{PacketEvent, PlayerConnection},
    play::PlayerJoinWorldEvent,
};

pub fn configuration_packets(
    receiver: Receiver<PacketEvent, (EntityId, &PlayerConnection)>,
    mut world_join_sender: Sender<PlayerJoinWorldEvent>,
) {
    let (entity, connection) = receiver.query;
    let PacketEvent {
        packet,
        protcol_state: state,
    } = receiver.event;
    if *state != ProtocolState::Configuration {
        return;
    }

    if let Some(_packet) = packet.decode::<ClientInformationC2SPacket>(*state) {
        let crabcraft_version = env!("CARGO_PKG_VERSION");
        connection.send_packet(ReportDetailsS2CPacket {
            details: vec![ReportDetail {
                title: "Server".to_owned(),
                description: format!("CrabCraft v{}", crabcraft_version),
            }],
        });

        connection.send_packet(ServerLinksS2CPacket {
            links: vec![ServerLink::Known {
                link: KnownServerLinkType::BugReport,
                url: "https://github.com/CrabCraftDev/CrabCraft".to_owned(),
            }],
        });

        // TODO: Current registry system is temporary. Will be fixed in #127
        for packet_data in [
            REGISTRY_DIMENSION_TYPE_PACKET,
            REGISTRY_BIOMES_PACKET,
            REGISTRY_DAMAGE_TYPE_PACKET,
            REGISTRY_PAINTING_VARIANT_PACKET,
            REGISTRY_WOLF_VARIANT_PACKET,
        ] {
            connection
                .packet_sender
                .send(RawPacket {
                    id: RegistryDataS2CPacket::get_id_for_state(ProtocolState::Configuration).unwrap(),
                    data: Bytes::from(packet_data),
                })
                .unwrap()
        }

        let mut data = BytesMut::new();
        "CrabCraft".to_owned().serialize(&mut data).unwrap();

        connection.send_packet(PluginMessageS2CPacket {
            channel: Identifier::minecraft("brand").unwrap(),
            data: data.freeze().into(),
        });

        connection.send_packet(KnownPacksS2CPacket {
            known_packs: vec![KnownPack {
                namespace: "minecraft".to_string(),
                id: "core".to_string(),
                version: SERVER_VERSION.to_string(),
            }],
        });
    }

    if let Some(_packet) = packet.decode::<KnownPacksC2SPacket>(*state) {
        connection.send_packet(FinishConfigurationS2CPacket);
    }

    if let Some(_packet) = packet.decode::<AcknowledgeFinishConfigurationC2SPacket>(*state) {
        *connection.protocol_state.write().unwrap() = ProtocolState::Play;
        world_join_sender.send_to(entity, PlayerJoinWorldEvent);
    }
}
