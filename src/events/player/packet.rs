use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use evenio::event::TargetedEvent;

#[derive(TargetedEvent, AsyncEvent, Debug)]
pub struct PacketEvent {
    pub packet: RawPacket,
    pub protocol_state: ProtocolState,
}
