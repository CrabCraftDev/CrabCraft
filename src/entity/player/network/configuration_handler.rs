use super::{connection::PacketEvent, play_handler::PlayerJoinWorldEvent};
use crate::{
    mc_server::COMPATIBLE_SERVER_VERSIONS, network::connection::NetworkConnection, DISCORD_INVITE_URL,
    GITHUB_ISSUES_URL,
};
use bytes::BytesMut;
use crab_protocol::packets::common::{
    ClientInformationC2SPacket, KnownServerLinkType, PluginMessageS2CPacket, ReportDetail, ReportDetailsS2CPacket,
    ServerLink, ServerLinksS2CPacket,
};
use crab_protocol::packets::configuration::RegistryDataEntry;
use crab_protocol::types::namespaced_key::MINECRAFT_NAMESPACE;
use crab_protocol::{
    packets::configuration::{
        AcknowledgeFinishConfigurationC2SPacket, FinishConfigurationS2CPacket, KnownPacksC2SPacket,
        KnownPacksS2CPacket, RegistryDataS2CPacket,
    },
    protocol_serializer::ProtocolSerializer,
    protocol_state::ProtocolState,
    types::{known_pack::KnownPack, namespaced_key::NamespacedKey},
};
use crab_registry::REGISTRY;
use evenio::{
    entity::EntityId,
    event::{Receiver, Sender},
};

pub fn configuration_packets(
    receiver: Receiver<PacketEvent, (EntityId, &NetworkConnection)>,
    mut world_join_sender: Sender<PlayerJoinWorldEvent>,
) {
    let (entity, connection) = receiver.query;
    let PacketEvent {
        packet,
        protocol_state: state,
    } = receiver.event;
    if *state != ProtocolState::Configuration {
        return;
    }

    if let Some(_packet) = packet.decode::<ClientInformationC2SPacket>(*state) {
        let crabcraft_version = env!("CARGO_PKG_VERSION");
        connection.send_packet(ReportDetailsS2CPacket {
            details: vec![ReportDetail {
                title: "Server".to_owned(),
                description: format!("CrabCraft v{crabcraft_version}"),
            }],
        });

        connection.send_packet(ServerLinksS2CPacket {
            links: vec![
                ServerLink::Known {
                    link: KnownServerLinkType::BugReport,
                    url: GITHUB_ISSUES_URL.to_owned(),
                },
                ServerLink::Known {
                    link: KnownServerLinkType::Community,
                    url: DISCORD_INVITE_URL.to_string(),
                },
            ],
        });

        let known_packs = COMPATIBLE_SERVER_VERSIONS
            .iter()
            .map(|version| KnownPack {
                namespace: MINECRAFT_NAMESPACE.to_string(),
                id: "core".to_string(),
                version: version.to_string(),
            })
            .collect();
        connection.send_packet(KnownPacksS2CPacket { known_packs });

        let registry_data = &REGISTRY.get().unwrap().data;
        for (registry_name, values) in registry_data.iter() {
            let data_entries = values
                .iter()
                .map(|value| RegistryDataEntry {
                    id: value.clone(),
                    data: None,
                })
                .collect();

            connection.send_packet(RegistryDataS2CPacket {
                registry_id: registry_name.clone(),
                entries: data_entries,
            });
        }

        let mut data = BytesMut::new();
        "CrabCraft".to_owned().serialize(&mut data).unwrap();

        connection.send_packet(PluginMessageS2CPacket {
            channel: NamespacedKey::minecraft("brand").unwrap(),
            data: data.freeze().into(),
        });
    }

    if let Some(_packet) = packet.decode::<KnownPacksC2SPacket>(*state) {
        connection.send_packet(FinishConfigurationS2CPacket);
    }

    if let Some(_packet) = packet.decode::<AcknowledgeFinishConfigurationC2SPacket>(*state) {
        connection.set_state(ProtocolState::Play);
        world_join_sender.send_to(entity, PlayerJoinWorldEvent);
    }
}
