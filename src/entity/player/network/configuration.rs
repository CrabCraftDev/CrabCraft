use bytes::{Bytes, BytesMut};
use crab_protocol::{
    packet::Packet,
    packets::configuration::{
        AcknowledgeFinishConfigurationC2SPacket, ClientInformationC2SPacket, FinishConfigurationS2CPacket,
        KnownPacksC2SPacket, KnownPacksS2CPacket, PluginMessageS2CPacket, RegistryDataS2CPacket,
    },
    protocol::ProtocolState,
    protocol_serializer::ProtocolSerializer,
    raw_packet::RawPacket,
    types::{identifier::Identifier, known_pack::KnownPack},
};
use evenio::{
    entity::EntityId,
    event::{Receiver, Sender},
};

use crate::SERVER_VERSION;

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

    if let Some(_packet) = packet.decode::<ClientInformationC2SPacket>() {
        // Minecraft defines all biomes, damage types, and dimension types in code.
        let registry_dimension_type_packet: &[u8] =
            include_bytes!("../../../../assets/packets/registry_dimension_type.bin");
        connection
            .packet_sender
            .send(RawPacket {
                id: RegistryDataS2CPacket::ID,
                data: Bytes::from(registry_dimension_type_packet),
            })
            .unwrap();
        let registry_damage_type_packet: &[u8] = include_bytes!("../../../../assets/packets/registry_damage_type.bin");
        connection
            .packet_sender
            .send(RawPacket {
                id: RegistryDataS2CPacket::ID,
                data: Bytes::from(registry_damage_type_packet),
            })
            .unwrap();
        let registry_biomes_packet: &[u8] = include_bytes!("../../../../assets/packets/registry_biomes.bin");
        connection
            .packet_sender
            .send(RawPacket {
                id: RegistryDataS2CPacket::ID,
                data: Bytes::from(registry_biomes_packet),
            })
            .unwrap();

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

    if let Some(_packet) = packet.decode::<KnownPacksC2SPacket>() {
        connection.send_packet(FinishConfigurationS2CPacket);
    }

    if let Some(_packet) = packet.decode::<AcknowledgeFinishConfigurationC2SPacket>() {
        *connection.protocol_state.write().unwrap() = ProtocolState::Play;
        world_join_sender.send_to(entity, PlayerJoinWorldEvent);
    }
}
