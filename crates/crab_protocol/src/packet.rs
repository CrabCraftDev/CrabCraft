use std::fmt::Debug;

use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState;

pub trait Packet: Sized + Debug + ProtocolSerializer {
    const STATE_IDS: &'static [(ProtocolState, i32)];

    fn get_id_for_state(check_state: ProtocolState) -> Option<i32> {
        Self::STATE_IDS
            .iter()
            .find_map(|(state, id)| if *state == check_state { Some(*id) } else { None })
    }
}
