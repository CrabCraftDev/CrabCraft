use bytes::{Buf, BufMut, Bytes, BytesMut};

// Accepts bytes continuously, and splits them into individual packets
pub struct PacketSplitter {
    buffer: BytesMut,
    state: SplitterState,
}

enum SplitterState {
    ReadingVarInt { value: i32, position: i8 },
    Collecting { needs: i32 },
}

use crab_protocol::{
    buf::{CONTINUE_BIT, SEGMENT_BITS},
    error::ProtocolError,
};
use SplitterState::*;

impl PacketSplitter {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
            state: SplitterState::ReadingVarInt { value: 0, position: 0 },
        }
    }

    pub fn split(&mut self, buffer: impl Buf) -> Result<Vec<Bytes>, ProtocolError> {
        self.buffer.put(buffer);

        let mut packets = Vec::new();

        while self.buffer.has_remaining() {
            match self.state {
                ReadingVarInt { value, position } => self.read_var_int(value, position)?,
                Collecting { needs } => {
                    if let Some(packet) = self.collect(needs) {
                        packets.push(packet);
                    };
                }
            }
        }

        Ok(packets)
    }

    fn read_var_int(&mut self, mut value: i32, mut position: i8) -> Result<(), ProtocolError> {
        let current_byte = self.buffer.get_i8();

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            self.state = Collecting { needs: value };
            return Ok(());
        }

        position += 7;

        if position > 31 {
            return Err(ProtocolError::TooBigVarInt(32, position + 1));
        }

        self.state = ReadingVarInt { value, position };
        Ok(())
    }

    fn collect(&mut self, needs: i32) -> Option<Bytes> {
        if (self.buffer.remaining() as i32) < needs {
            return None;
        }
        self.state = ReadingVarInt { value: 0, position: 0 };
        Some(self.buffer.split_to(needs as usize).into())
    }
}

impl Default for PacketSplitter {
    fn default() -> Self {
        Self::new()
    }
}
