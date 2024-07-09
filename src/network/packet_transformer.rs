use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

use bytes::{BufMut, Bytes, BytesMut};
use crab_protocol::{
    buf::{ReadByteBuf, WriteByteBuf},
    error::ProtocolError,
    raw_packet::RawPacket,
};

#[derive(Debug, Clone)]
pub struct PacketTransformer {
    compression_threshold: Arc<AtomicI32>,
    // Note: Until we have encryption support this doesn't need to be thread safe because it's always false
    #[allow(dead_code)]
    encryption: bool,
}

impl PacketTransformer {
    pub fn new() -> Self {
        Self {
            compression_threshold: Arc::new(AtomicI32::new(-1)),
            encryption: false,
        }
    }

    pub fn decode(&self, mut buffer: Bytes) -> Result<RawPacket, ProtocolError> {
        buffer = self.decrypt(buffer);
        buffer = self.decompress(buffer)?;

        let id = buffer.get_var_int()?;
        Ok(RawPacket::new(id, buffer))
    }

    pub fn encode(&self, packet: RawPacket) -> Result<Bytes, ProtocolError> {
        let mut bytes = packet.to_bytes();
        bytes = self.compress(bytes);
        bytes = self.encrypt(bytes);

        let mut result = BytesMut::new();
        result.put_var_int(bytes.len() as i32);
        result.put(bytes);

        Ok(result.into())
    }

    fn compress(&self, buffer: Bytes) -> Bytes {
        if self.compression_threshold.load(Ordering::SeqCst) < 0 {
            return buffer;
        }
        todo!()
    }

    fn decompress(&self, buffer: Bytes) -> Result<Bytes, ProtocolError> {
        if self.compression_threshold.load(Ordering::SeqCst) < 0 {
            return Ok(buffer);
        }
        todo!()
    }

    fn decrypt(&self, buffer: Bytes) -> Bytes {
        if !self.encryption {
            return buffer;
        }
        todo!()
    }

    fn encrypt(&self, buffer: Bytes) -> Bytes {
        if !self.encryption {
            return buffer;
        }
        todo!()
    }
}

impl Default for PacketTransformer {
    fn default() -> Self {
        Self::new()
    }
}
