use std::io::Read;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::{bufread::ZlibDecoder, read::ZlibEncoder, Compression};

use super::{
    packet_byte_buf::{read_byte_buf::ReadByteBuf, write_byte_buf::WriteByteBuf},
    packet_splitter::PacketSplitter,
};

pub struct PacketTransformer {
    compression_threshold: i32,
    #[allow(dead_code)]
    encryption: bool,
    splitter: PacketSplitter,
}

impl PacketTransformer {
    pub fn new() -> Self {
        Self {
            compression_threshold: -1,
            encryption: false,
            splitter: PacketSplitter::new(),
        }
    }

    pub fn decode_packets(&mut self, buf: Bytes) -> Vec<(i32, Bytes)> {
        let splitted = self.splitter.split(buf);
        let mut packets = Vec::new();
        for mut packet in splitted {
            if self.compression_threshold > 0 {
                packet = self.decompress(packet);
            }

            let id = packet.get_var_int();
            let data = packet;
            packets.push((id, data))
        }
        packets
    }

    #[allow(dead_code)]
    pub fn encode(&mut self, mut packet: Bytes) -> Bytes {
        if self.compression_threshold > 0 {
            packet = self.compress(packet);
        }

        let mut result = BytesMut::new();
        result.put_var_int(packet.len() as i32);
        result.put(packet);

        result.into()
    }

    #[allow(dead_code)]
    fn compress(&mut self, mut buf: Bytes) -> Bytes {
        let mut compressed_size = 0;

        if buf.len() as i32 > self.compression_threshold {
            let mut encoder = ZlibEncoder::new(buf.reader(), Compression::default());
            let mut vec_buf = Vec::new();
            encoder.read_to_end(&mut vec_buf).unwrap();
            buf = Bytes::from(vec_buf);
            compressed_size = buf.len();
        }

        let mut result = BytesMut::new();
        result.put_var_int(compressed_size as i32);
        result.put(buf);

        result.into()
    }

    fn decompress(&mut self, mut buf: Bytes) -> Bytes {
        let size = buf.get_var_int();
        if size != 0 {
            let mut decoder = ZlibDecoder::new(buf.reader());
            let mut vec_buf = Vec::new();
            decoder.read_to_end(&mut vec_buf).unwrap();
            buf = Bytes::from(vec_buf);
        }
        buf
    }

    #[allow(dead_code)]
    fn decrypt(&mut self, _: Bytes) -> Bytes {
        todo!()
    }

    #[allow(dead_code)]
    fn encrypt(&mut self, _: Bytes) -> Bytes {
        todo!()
    }
}

impl Default for PacketTransformer {
    fn default() -> Self {
        Self::new()
    }
}
