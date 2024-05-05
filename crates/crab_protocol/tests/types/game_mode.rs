use bytes::Bytes;

use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::types::game_mode::{GameMode, OptionalGameMode};

#[test]
pub fn serialize_optional_gamemode() {
    let mut buf = Vec::new();
    ProtocolSerializer::serialize(&OptionalGameMode(None), &mut buf).unwrap();
    ProtocolSerializer::serialize(&OptionalGameMode(Some(GameMode::Creative)), &mut buf).unwrap();

    assert_eq!(buf, &[-1i8 as u8, 1]);
}

#[test]
pub fn deserialize_optional_gamemode() {
    let mut buf = Bytes::from(vec![-1i8 as u8, 1]);

    let no_game_mode = OptionalGameMode::deserialize(&mut buf).unwrap();
    let creative = OptionalGameMode::deserialize(&mut buf).unwrap();

    assert_eq!(no_game_mode, None.into());
    assert_eq!(creative, Some(GameMode::Creative).into());
}
