use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_protocol::{
    buf::{CONTINUE_BIT, SEGMENT_BITS},
    error::ProtocolError,
};
use SplitterState::*;

/// Accepts bytes continuously, and splits them into individual packets
/// Basiaclly a fancy replacement for tokio's read_exact, which we can't use because it's not cancellation safe
#[derive(Default)]
pub struct PacketSplitter {
    buffer: BytesMut,
    state: SplitterState,
}

#[derive(Debug)]
enum SplitterState {
    ReadingVarInt { value: i32, position: i8 },
    Collecting { needs: i32 },
}

impl PacketSplitter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn split_bytes(&mut self, buffer: impl Buf) -> Result<Vec<Bytes>, ProtocolError> {
        self.buffer.put(buffer);

        let mut packets = Vec::new();

        while self.buffer.has_remaining() {
            match self.state {
                ReadingVarInt { value, position } => self.read_var_int(value, position)?,
                Collecting { needs } => {
                    match self.collect(needs) {
                        Some(packet) => packets.push(packet),
                        None => break,
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

impl Default for SplitterState {
    fn default() -> Self {
        ReadingVarInt { value: 0, position: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crab_protocol::buf::WriteByteBuf;

    use super::*;

    #[test]
    fn test_packet_splitter() {
        let mut splitter = PacketSplitter::new();

        let mut bytes = BytesMut::new();
        let data1 = b"PacketSplitter";
        let data2 = b"CrabCraft!";
        bytes.put_var_int(data1.len().try_into().unwrap());
        bytes.extend_from_slice(data1);
        bytes.put_var_int(data2.len().try_into().unwrap());
        bytes.extend_from_slice(data2);

        let bytes2 = bytes.split_off(7);

        let result1 = splitter.split_bytes(bytes).unwrap();
        let result2 = splitter.split_bytes(bytes2).unwrap();

        assert_eq!(result1.len(), 0);
        assert_eq!(result2.len(), 2);
        assert_eq!(&result2[0][..], data1);
        assert_eq!(&result2[1][..], data2);
    }
}
