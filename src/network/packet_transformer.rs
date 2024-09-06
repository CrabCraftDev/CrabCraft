use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::packet::Packet;
use crab_protocol::packets::login::SetCompressionS2CPacket;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use flate2::{bufread::ZlibDecoder, read::ZlibEncoder, Compression};
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub const COMPRESSION_THRESHOLD: i32 = 256;

#[derive(Clone, Default)]
pub struct PacketTransformer {
    pub compressed: Arc<AtomicBool>,
    pub encrypted: Arc<AtomicBool>,
}

impl PacketTransformer {
    pub fn encode(&self, packet: RawPacket, state: ProtocolState) -> Result<Bytes, ProtocolError> {
        let mut bytes = packet.to_bytes();
        bytes = self.try_compressing(bytes);
        bytes = self.try_encrypting(bytes);

        let is_correct_packet_id = SetCompressionS2CPacket::get_id_for_state(state) == Some(packet.id);
        if is_correct_packet_id && state == ProtocolState::Login {
            self.compressed.store(true, Ordering::SeqCst);
        }

        let mut result = BytesMut::new();
        result.put_var_int(bytes.len() as i32);
        result.put(bytes);

        Ok(result.into())
    }

    pub fn decode(&self, buffer: Vec<u8>) -> Result<RawPacket, ProtocolError> {
        let mut bytes = Bytes::from(buffer);
        bytes = self.try_decrypting(bytes);
        bytes = self.try_decompressing(bytes)?;

        let id = bytes.get_var_int()?;
        Ok(RawPacket::new(id, bytes))
    }

    fn try_compressing(&self, mut buffer: Bytes) -> Bytes {
        if !self.compressed.load(Ordering::SeqCst) {
            return buffer;
        }
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

    fn try_decompressing(&self, mut buffer: Bytes) -> Result<Bytes, ProtocolError> {
        if !self.compressed.load(Ordering::SeqCst) {
            return Ok(buffer);
        }

        let data_length = buffer.get_var_int()?;
        if data_length == 0 {
            return Ok(buffer);
        }

        let mut decoder = ZlibDecoder::new(buffer.reader());
        let mut vec_buf = Vec::new();
        decoder.read_to_end(&mut vec_buf)?;
        Ok(Bytes::from(vec_buf))
    }

    fn try_encrypting(&self, buffer: Bytes) -> Bytes {
        if !self.encrypted.load(Ordering::SeqCst) {
            return buffer;
        }

        todo!("Missing try_encrypting function")
    }

    fn try_decrypting(&self, buffer: Bytes) -> Bytes {
        if !self.encrypted.load(Ordering::SeqCst) {
            return buffer;
        }

        todo!("Missing try_decrypting function")
    }
}
