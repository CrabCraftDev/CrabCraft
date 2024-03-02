use bytes::{Buf, BufMut, Bytes, BytesMut};

// Collects bytes from a stream, and splits them into individual packets
pub struct PacketSplitter {
    buffer: BytesMut,
    state: SplitterState,
}

enum SplitterState {
    ReadingVarInt { value: i32, position: i8 },
    Collecting { needs: i32 },
}

use SplitterState::*;

use crate::{
    network::packet_byte_buf::read_byte_buf::ReadByteBuf,
    utils::network::{CONTINUE_BIT, SEGMENT_BITS},
};

impl PacketSplitter {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
            state: SplitterState::ReadingVarInt { value: 0, position: 0 },
        }
    }

    pub fn split(&mut self, buf: Bytes) -> Vec<Bytes> {
        self.buffer.put(buf);

        let mut packets = Vec::new();

        while self.buffer.has_remaining() {
            match self.state {
                ReadingVarInt { value, position } => self.read_var_int(value, position),
                Collecting { needs } => {
                    if let Some(packet) = self.collect(needs) {
                        packets.push(packet);
                    }
                }
            }
        }

        packets
    }

    fn read_var_int(&mut self, mut value: i32, mut position: i8) {
        let current_byte = self.buffer.get_byte();

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            // Got a value
            self.state = Collecting { needs: value };
            return;
        }

        position += 7;

        assert!(position < 32, "Varint exceeds 32 bits");
        // Needs more bytes, update the state
        self.state = ReadingVarInt { value, position }
    }

    fn collect(&mut self, needs: i32) -> Option<Bytes> {
        if self.buffer.remaining() as i32 >= needs {
            self.state = ReadingVarInt { value: 0, position: 0 };
            return Some(self.buffer.split_to(needs as usize).into());
        }
        None
    }
}

impl Default for PacketSplitter {
    fn default() -> Self {
        Self::new()
    }
}
