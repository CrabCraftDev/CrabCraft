use std::io::Read;
use std::io::Write;

use bytes::{Bytes, BytesMut};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use log::debug;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::network::handlers::connection_handler::ConnectionHandler;
use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::network::packet_handler::PacketHandler;
use crate::utils::network::{get_var_int_from_stream, to_var_int};

pub struct TcpServer {
    address: String,
}

#[derive(Debug, PartialEq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

pub struct ConnectionData {
    pub(crate) state: ProtocolState,
}

impl TcpServer {
    pub fn new(address: String) -> TcpServer {
        TcpServer { address }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();

        loop {
            match listener.accept().await {
                Ok((stream, address)) => {
                    debug!("Connection established {address}!");
                    stream.set_nodelay(true).unwrap();
                    tokio::spawn(Self::handle_connection(stream));
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    pub async fn send(stream: &mut TcpStream, bytes: Bytes) {
        stream.write_all(&to_var_int(bytes.len() as i32)).await.unwrap();
        stream.write_all(&bytes).await.unwrap();
    }

    #[allow(dead_code)]
    pub async fn send_compressed(stream: &mut TcpStream, bytes: Bytes, compression_threshold: i32) {
        let should_compress = compression_threshold < 0 || compression_threshold < bytes.len() as i32;
        let mut data_length = if should_compress {
            to_var_int(0)
        } else {
            to_var_int(bytes.len() as i32)
        };

        let mut data;
        if should_compress {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&bytes).expect("Couldn't compress packet!");
            data = encoder.finish().unwrap();
        } else {
            data = Vec::from(bytes)
        }

        data_length.append(&mut data);
        Self::send(stream, Bytes::from(data_length)).await;
    }

    #[allow(dead_code)]
    pub async fn decompress_packet(mut bytes: Bytes) -> Bytes {
        let mut new_bytes = Vec::new();
        let data_length = bytes.get_var_int();
        if data_length == 0 {
            return Bytes::from(bytes.get_bytes(data_length));
        }

        let compressed = &*bytes.get_bytes(data_length);
        let mut decoder = ZlibDecoder::new(compressed);
        decoder.read_to_end(&mut new_bytes).unwrap();

        Bytes::from(new_bytes)
    }

    async fn handle_connection(mut stream: TcpStream) {
        let mut handlers: Vec<Box<dyn PacketHandler + Send>> = vec![Box::new(ConnectionHandler)];
        let mut connection_data = ConnectionData {
            state: ProtocolState::Handshake,
        };

        while let Ok(len) = get_var_int_from_stream(&mut stream).await {
            let mut buffer = vec![0; len as usize];

            if let Ok(0) = stream.read_exact(&mut buffer).await {
                break;
            }

            let mut_bytes = BytesMut::from(buffer.as_slice());
            let mut byte = mut_bytes.clone().freeze();

            let packet_id = byte.get_var_int();

            for handler in &mut handlers {
                handler.handle_packet(&mut stream, &mut connection_data, byte.clone(), packet_id);
            }
        }
        debug!("Connection closed {:?}!", stream.peer_addr().unwrap());
    }
}
