use crate::config::manager::ConfigManager;
use crate::events::player::packet::PacketEvent;
use crate::events::player::player_join_world::PlayerJoinWorldEvent;
use crate::{
    DISCORD_INVITE_URL, GITHUB_ISSUES_URL, mc_server::COMPATIBLE_SERVER_VERSIONS,
    network::connection::NetworkConnection,
};
use bytes::BytesMut;
use crab_protocol::packets::common::{
    ClientInformation, ClientInformationC2SPacket, KnownServerLinkType, PluginMessageS2CPacket, ReportDetail,
    ReportDetailsS2CPacket, ServerLink, ServerLinksS2CPacket, Tag, UpdateTagsS2CPacket,
};
use crab_protocol::packets::configuration::RegistryDataEntry;
use crab_protocol::{
    packets::configuration::{
        AcknowledgeFinishConfigurationC2SPacket, FinishConfigurationS2CPacket, KnownPacksC2SPacket,
        KnownPacksS2CPacket, RegistryDataS2CPacket,
    },
    protocol_serializer::ProtocolSerializer,
    protocol_state::ProtocolState,
    types::known_pack::KnownPack,
};
use crab_registry::REGISTRY;
use crab_text::utils::namespaced_key::{MINECRAFT_NAMESPACE, NamespacedKey};
use evenio::event::Insert;
use evenio::{
    entity::EntityId,
    event::{Receiver, Sender},
};
use std::collections::HashMap;
use std::str::FromStr;

pub fn configuration_packets(
    receiver: Receiver<PacketEvent, (EntityId, &NetworkConnection)>,
    sender: Sender<(PlayerJoinWorldEvent, Insert<ClientInformation>)>,
) {
    let (entity, connection) = receiver.query;
    let PacketEvent {
        packet,
        protocol_state: state,
    } = receiver.event;
    if *state != ProtocolState::Configuration {
        return;
    }

    if let Some(packet) = packet.decode::<ClientInformationC2SPacket>(*state) {
        sender.insert(entity, packet as ClientInformation);

        if ConfigManager::get().main.mention_crabcraft_usage {
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
        }

        let known_packs = COMPATIBLE_SERVER_VERSIONS
            .iter()
            .map(|version| KnownPack {
                namespace: MINECRAFT_NAMESPACE.to_string(),
                id: "core".to_string(),
                version: version.to_string(),
            })
            .collect();
        connection.send_packet(KnownPacksS2CPacket { known_packs });

        // Note: Tags implementation is also temporary. Will be fixed in #163
        let mut tags = HashMap::new();
        tags.insert(
            NamespacedKey::minecraft("fluid").unwrap(),
            vec![
                Tag {
                    name: NamespacedKey::minecraft("water").unwrap(),
                    entries: vec![2, 1],
                },
                Tag {
                    name: NamespacedKey::minecraft("lava").unwrap(),
                    entries: vec![4, 3],
                },
            ],
        );
        tags.insert(
            NamespacedKey::minecraft("worldgen/biome").unwrap(),
            vec![
                Tag {
                    name: NamespacedKey::minecraft("is_badlands").unwrap(),
                    entries: vec![0],
                },
                Tag {
                    name: NamespacedKey::minecraft("is_jungle").unwrap(),
                    entries: vec![0],
                },
                Tag {
                    name: NamespacedKey::minecraft("is_savanna").unwrap(),
                    entries: vec![0],
                },
            ],
        );
        connection.send_packet(UpdateTagsS2CPacket { tags });

        // FIXME: Verify the minimal set of registries the client actually needs
        //       I don't think that sending all registries with namespace minecraft is a good idea
        let registry_data = &REGISTRY.load();
        for (registry_name, values) in registry_data.all_registries() {
            if !registry_name.starts_with("minecraft:") {
                continue;
            }

            let data_entries = values
                .keys()
                .map(|value| RegistryDataEntry {
                    id: value.clone(),
                    data: None,
                })
                .collect();

            connection.send_packet(RegistryDataS2CPacket {
                registry_id: NamespacedKey::from_str(registry_name).unwrap(),
                entries: data_entries,
            });
        }

        let mut data = BytesMut::new();
        let server_brand = ConfigManager::get().main.server_brand.to_owned();
        server_brand.serialize(&mut data).unwrap();

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
        sender.send_to(entity, PlayerJoinWorldEvent);
    }
}
