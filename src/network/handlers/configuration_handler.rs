use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use crab_nbt::Nbt;
use log::info;

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::packets::configuration::{
    AcknowledgeFinishConfigurationC2SPacket, ClientInformationC2SPacket, FinishConfigurationS2CPacket,
    PluginMessageS2CPacket, RegistryDataS2CPacket,
};
use crab_protocol::packets::play::{
    GameEventS2CPacket, GameJoinS2CPacket, SetDefaultSpawnPositionS2CPacket, SynchronizePlayerPositionS2CPacket,
    SynchronizePositionFlags,
};

use crab_protocol::protocol::ProtocolState;

use crab_protocol::raw_packet::RawPacket;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::position::Position;

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::utils::errors::Error;

pub struct ConfigurationHandler;

#[async_trait]
impl PacketHandler for ConfigurationHandler {
    async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(_packet) = raw_packet.decode::<ClientInformationC2SPacket>() {
            let binary_registry_data: &[u8] = include_bytes!("../../../assets/registry-data.nbt");
            connection
                .send_packet(RegistryDataS2CPacket {
                    registry_data: Nbt::read(&mut Bytes::from(binary_registry_data))?,
                })
                .await;

            let mut data = BytesMut::new();
            data.put_string("CrabCraft");

            connection
                .send_packet(PluginMessageS2CPacket {
                    channel: Identifier::minecraft("brand")?,
                    data: data.freeze().into(),
                })
                .await;

            connection.send_packet(FinishConfigurationS2CPacket).await;
        }

        if let Some(_packet) = raw_packet.decode::<AcknowledgeFinishConfigurationC2SPacket>() {
            connection.state = ProtocolState::Play;

            connection
                .send_packet(GameJoinS2CPacket {
                    entity_id: 0,
                    is_hardcore: false,
                    dimension_names: vec![Identifier::minecraft("overworld")?],
                    max_players: 100,
                    view_distance: 16,
                    simulation_distance: 20,
                    reduced_debug_info: false,
                    enable_respawn_screen: true,
                    do_limited_crafting: false,
                    dimension_type: Identifier::minecraft("overworld")?,
                    dimension_name: Identifier::minecraft("overworld")?,
                    hashed_seed: 99887766,
                    game_mode: GameMode::Creative,
                    previous_game_mode: None.into(),
                    is_debug_world: false,
                    is_flat_world: true,
                    last_death_location: None,
                    portal_cooldown: 20,
                })
                .await;

            connection
                .send_packet(SetDefaultSpawnPositionS2CPacket {
                    location: BlockPosition { x: 0, y: 0, z: 0 },
                    angle: 0.0,
                })
                .await;

            connection
                .send_packet(SynchronizePlayerPositionS2CPacket {
                    position: Position::new(0.0, 0.0, 0.0),
                    yaw: 0.0,
                    pitch: 0.0,
                    flags: SynchronizePositionFlags::new(),
                    teleport_id: 2137,
                })
                .await;

            connection
                .send_packet(GameEventS2CPacket { event: 13, value: 0.0 })
                .await;

            info!("Player joined!");
        }

        Ok(())
    }
}
