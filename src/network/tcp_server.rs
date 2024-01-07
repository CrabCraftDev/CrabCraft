use std::sync::Arc;

use bytes::BytesMut;
use log::debug;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task::JoinHandle,
};

use crate::network::handlers::global_handler::GlobalHandler;
use crate::network::handlers::handshake_handler::HandshakeHandler;
use crate::network::packet_byte_buffer::ReadByteBuf;
use crate::network::packet_handler::PacketHandler;

pub struct TcpServer {
    address: String,
    event_handlers: Arc<Mutex<Vec<Box<dyn PacketHandler + Send>>>>,
    pub thread: Option<JoinHandle<()>>,
}

impl TcpServer {
    pub async fn start(address: String) -> Arc<Mutex<TcpServer>> {
        let event_handlers: Vec<Box<dyn PacketHandler + Send>> = vec![Box::new(HandshakeHandler), Box::new(GlobalHandler)];

        let server = Arc::new(Mutex::new(TcpServer {
            address,
            event_handlers: Arc::new(Mutex::new(event_handlers)),
            thread: None,
        }));

        let listener = TcpListener::bind(&server.lock().await.address).await.unwrap();

        let cloned_server = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, address)) => {
                        debug!("Connection established {}!", address);
                        let cloned_server = Arc::clone(&cloned_server);
                        tokio::spawn(async move {
                            cloned_server.lock().await.handle_connection(stream).await;
                        });
                    }
                    Err(_) => continue,
                }
            }
        });

        server.lock().await.thread = Some(handle);

        server
    }

    pub async fn send(stream: &mut TcpStream, bytes: Vec<u8>) {
        stream.write_all(&bytes).await.expect("Couldn't write to stream!");
    }

    async fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = BytesMut::new();
        stream.read_buf(&mut buffer).await.expect("Invalid stream!");
        let bytes = buffer.freeze();

        let mut byte = bytes.clone();

        let _length = byte.get_var_int();
        let packet_id = byte.get_var_int();
        debug!("Received packet ({}): {:?}", packet_id, byte);

        for handler in self.event_handlers.lock().await.iter() {
            handler.handle_packet(&mut stream, byte.clone(), packet_id);
        }
    }
}
