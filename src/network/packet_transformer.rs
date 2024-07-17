use std::sync::{
    atomic::{AtomicBool, Ordering},
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
    compression_enabled: Arc<AtomicBool>,
    // Note: Until we have encryption support this doesn't need to be thread safe because it's always false
    encryption_enabled: bool,
}

impl PacketTransformer {
    pub fn new() -> Self {
        Self::default()
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
        if !self.compression_enabled.load(Ordering::SeqCst) {
            return buffer;
        }
        todo!()
    }

    fn decompress(&self, buffer: Bytes) -> Result<Bytes, ProtocolError> {
        if !self.compression_enabled.load(Ordering::SeqCst) {
            return Ok(buffer);
        }
        todo!()
    }

    fn decrypt(&self, buffer: Bytes) -> Bytes {
        if !self.encryption_enabled {
            return buffer;
        }
        todo!()
    }

    fn encrypt(&self, buffer: Bytes) -> Bytes {
        if !self.encryption_enabled {
            return buffer;
        }
        todo!()
    }
}

impl Default for PacketTransformer {
    fn default() -> Self {
        Self {
            compression_enabled: Arc::new(AtomicBool::new(false)),
            encryption_enabled: false,
        }
    }
}
