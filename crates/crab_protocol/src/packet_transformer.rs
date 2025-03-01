use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_protocol::packet::get_id_for_state;
use crab_protocol::packets::login::SetCompressionS2CPacket;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::types::varint::VarInt;
use crab_protocol::{
    buf::{ReadByteBuf, WriteByteBuf},
    error::ProtocolError,
    raw_packet::RawPacket,
};
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use libdeflater::Decompressor;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::AsyncWriteExt;

pub const COMPRESSION_THRESHOLD: i32 = 256;

#[derive(Debug, Default, Clone)]
pub struct PacketTransformer {
    compression_enabled: Arc<AtomicBool>,
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
        if self.compressing() {
            buffer = self.decompress(buffer)?;
        }

        let id = buffer.get_var_int()?;
        Ok(RawPacket::new(id, buffer))
    }

    /// Compresses, encrypts, encodes and writes packet to stream
    /// Note: It automatically listens to packets end enables compression
    pub async fn encode_and_write(
        &mut self,
        packet: RawPacket,
        state: ProtocolState,
        read_half: &mut (impl AsyncWriteExt + Unpin),
    ) -> Result<(), ProtocolError> {
        let mut bytes = packet.to_bytes();

        if self.compressing() {
            bytes = self.compress(bytes);
        }
        if self.encryption_enabled {
            bytes = self.encrypt(bytes);
        }

        if !self.compressing() {
            let is_correct_packet_id = get_id_for_state::<SetCompressionS2CPacket>(state) == Some(packet.id);
            if is_correct_packet_id && state == ProtocolState::Login {
                self.compression_enabled.store(true, Ordering::SeqCst);
            }
        }

        read_half.write_all(&VarInt(bytes.len() as i32).as_bytes()).await?;
        read_half.write_all(&bytes).await?;

        Ok(())
    }

    fn compress(&self, mut buffer: Bytes) -> Bytes {
        let mut uncompressed_size = 0;
        if buffer.len() as i32 >= COMPRESSION_THRESHOLD {
            uncompressed_size = buffer.len();
            let mut encoder = ZlibEncoder::new(buffer.reader(), Compression::fast());
            let mut vec_buf = Vec::new();
            encoder.read_to_end(&mut vec_buf).unwrap();
            buffer = Bytes::from(vec_buf);
        }

        let mut result = BytesMut::new();
        result.put_var_int(uncompressed_size as i32);
        result.put_slice(&buffer);

        result.into()
    }

    fn decompress(&self, mut buffer: Bytes) -> Result<Bytes, ProtocolError> {
        let data_length = buffer.get_var_int()?;
        if data_length == 0 {
            return Ok(buffer);
        }

        let mut output = vec![0; data_length as usize];
        let mut decompress = Decompressor::new();
        decompress.zlib_decompress(&buffer, &mut output)?;

        if output.len() != data_length as usize {
            return Err(ProtocolError::DecompressedSizeMismatch(
                output.len(),
                data_length as usize,
            ));
        }

        Ok(Bytes::from(output))
    }

    fn decrypt(&self, _buffer: Bytes) -> Bytes {
        todo!("Will be implemented in #22")
    }

    fn encrypt(&self, _buffer: Bytes) -> Bytes {
        todo!("Will be implemented in #22")
    }

    fn compressing(&self) -> bool {
        self.compression_enabled.load(Ordering::SeqCst)
    }
}
