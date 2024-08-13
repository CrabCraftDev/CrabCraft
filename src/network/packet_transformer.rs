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
        if self.encryption_enabled {
            buffer = self.decrypt(buffer);
        }
        if self.compression_enabled.load(Ordering::SeqCst) {
            buffer = self.decompress(buffer)?;
        }

        let id = buffer.get_var_int()?;
        Ok(RawPacket::new(id, buffer))
    }

    pub fn encode(&self, packet: RawPacket) -> Result<Bytes, ProtocolError> {
        let mut bytes = packet.to_bytes();
        if self.compression_enabled.load(Ordering::SeqCst) {
            bytes = self.compress(bytes);
        }
        if self.encryption_enabled {
            bytes = self.encrypt(bytes);
        }

        let mut result = BytesMut::new();
        result.put_var_int(bytes.len() as i32);
        result.put(bytes);

        Ok(result.into())
    }

    fn compress(&self, _buffer: Bytes) -> Bytes {
        todo!()
    }

    fn decompress(&self, _buffer: Bytes) -> Result<Bytes, ProtocolError> {
        todo!()
    }

    fn decrypt(&self, _buffer: Bytes) -> Bytes {
        todo!()
    }

    fn encrypt(&self, _buffer: Bytes) -> Bytes {
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
