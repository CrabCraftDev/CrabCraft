use std::fmt::Debug;

use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState;

pub trait Packet: Sized + Debug + ProtocolSerializer {
    const STATE_IDS: &'static [(ProtocolState, i32)];
}

pub const fn get_id_for_state<P: Packet>(check_state: ProtocolState) -> Option<i32> {
    let len = P::STATE_IDS.len();
    let mut index = 0;
    while index < len {
        let (state, id) = P::STATE_IDS[index];
        if state as isize == check_state as isize {
            return Some(id);
        }
        index += 1;
    }
    None
}
