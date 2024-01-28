use std::io::Write;
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use log::{debug, error};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use std::io::Read;

use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::network::packet_handler::PacketHandler;
use crate::utils::network::to_var_int;

pub struct TcpServer {
    address: String,
    pub handlers: Arc<Mutex<Vec<Box<dyn PacketHandler + Send>>>>,
}

impl TcpServer {
    pub fn new(address: String, handlers: Vec<Box<dyn PacketHandler + Send>>) -> TcpServer {
        TcpServer {
            address,
            handlers: Arc::new(Mutex::new(handlers)),
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();

        loop {
            match listener.accept().await {
                Ok((socket, address)) => {
                    let handlers_clone = Arc::clone(&self.handlers);
                    debug!("Connection established {}!", address);
                    tokio::spawn(Self::handle_connection(socket, handlers_clone));
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
    pub async fn send_compressed(stream: &mut TcpStream, bytes: Bytes) {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bytes).expect("TODO: panic message");
        let compressed = encoder.finish().unwrap();

        let data_length = to_var_int(bytes.len() as i32);
        stream.write_all(&to_var_int((data_length.len() + compressed.len()) as i32)).await.unwrap(); // Packet length
        stream.write_all(&data_length).await.unwrap(); // Data length
        stream.write_all(&compressed).await.unwrap();
    }
    /// # Arguments
    /// * `bytes`: The bytes that shall only be read after reading the whole packet length.
    /// returns: Decompressed bytes
    /// # Example
    /// ```
    /// let _length = bytes.get_var_int();
    /// bytes = Self::decompress_packet(byte);
    /// ```
    #[allow(dead_code)]
    pub async fn decompress_packet(mut bytes: Bytes) -> Bytes {
        let mut new_bytes = Vec::new();
        let data_length = bytes.get_var_int();

        let compressed = &*bytes.get_bytes(data_length);
        let mut decoder = ZlibDecoder::new(compressed);
        decoder.read_to_end(&mut new_bytes).unwrap();

        return Bytes::from(new_bytes);
    }

    async fn handle_connection(mut stream: TcpStream, handlers: Arc<Mutex<Vec<Box<dyn PacketHandler + Send>>>>) {
        let mut buf = [0; 1024];

        loop {
            match stream.read(&mut buf).await {
                Ok(0) => {
                    debug!("Connection closed {:?}!", stream.peer_addr().unwrap());
                    break;
                }
                Ok(n) => {
                    let mut_bytes = BytesMut::from(&buf[0..n]);
                    let mut bytes = mut_bytes.freeze();

                    let _length = bytes.get_var_int();
                    // use bytes = Self::decompress_packet(byte); to decompress packet
                    let packet_id = bytes.get_var_int();
                    debug!("Received packet ({}): {:?}", packet_id, BytesMut::from(&buf[0..n]));

                    let handlers = handlers.lock().await;
                    for handler in handlers.iter() {
                        handler.handle_packet(&mut stream, bytes.clone(), packet_id);
                    }
                }
                Err(e) => {
                    error!("Error reading from socket: {}", e);
                    break;
                }
            }
        }
    }
}