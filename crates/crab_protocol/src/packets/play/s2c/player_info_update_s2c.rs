use bytes::Bytes;
use std::collections::HashMap;
use uuid::Uuid;

use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::bit_set::FixedBitSet;
use crate::types::game_mode::GameMode;
use crate::types::property::Property;
use crab_macros::Packet;
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_text::text::Text;

#[derive(Packet, Debug, Clone)]
#[packet(name = "player_info_update", id = 0x40, state = Play)]
pub struct PlayerInfoUpdateS2CPacket {
    pub actions: FixedBitSet<8>,
    pub player_actions: HashMap<Uuid, Vec<PlayerInfoAction>>,
}

impl ProtocolSerializer for PlayerInfoUpdateS2CPacket {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        self.actions.serialize(buf)?;
        buf.put_var_int(self.player_actions.len() as i32);
        for (uuid, actions) in self.player_actions.iter() {
            uuid.serialize(buf)?;
            for action in actions {
                match action {
                    PlayerInfoAction::AddPlayer { name, properties } => {
                        name.serialize(buf)?;
                        properties.serialize(buf)?;
                    }
                    PlayerInfoAction::InitialiseChat(_) => todo!(),
                    PlayerInfoAction::UpdateGameMode(game_mode) => game_mode.serialize(buf)?,
                    PlayerInfoAction::UpdateListed(listed) => listed.serialize(buf)?,
                    PlayerInfoAction::UpdateLatency(latency) => buf.put_var_int(*latency),
                    PlayerInfoAction::UpdateDisplayName(name) => name.serialize(buf)?,
                    PlayerInfoAction::UpdateListPriority(priority) => priority.serialize(buf)?,
                    PlayerInfoAction::UpdateHatVisibility(visible) => visible.serialize(buf)?,
                }
            }
        }

        Ok(())
    }

    fn deserialize(_buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub enum PlayerInfoAction {
    AddPlayer { name: String, properties: Vec<Property> },
    InitialiseChat(Option<(Uuid, i64, Bytes, Bytes)>),
    UpdateGameMode(GameMode),
    UpdateListed(bool),
    UpdateLatency(i32),
    UpdateDisplayName(Option<Text>),
    UpdateListPriority(i32),
    UpdateHatVisibility(bool),
}
