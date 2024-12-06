use std::fmt::Debug;

use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState;

pub trait Packet: Sized + Debug + ProtocolSerializer {
    /// Name of packet used in Notchian server
    /// Used for debugging, updating to newer Minecraft versions etc
    const NAME: &'static str;
    const STATE_IDS: &'static [(ProtocolState, i32)];
}

pub const fn get_id_for_state<P: Packet>(check_state: ProtocolState) -> Option<i32> {
    let length = P::STATE_IDS.len();
    let mut index = 0;
    while index < length {
        let (state, id) = P::STATE_IDS[index];
        if state as isize == check_state as isize {
            return Some(id);
        }
        index += 1;
    }
    None
}
