use std::sync::Arc;

use bytes::BytesMut;
use chrono::Local;
use log::{debug, error};
use tokio::io::ReadHalf;
use tokio::sync::Mutex;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::network::connection::Connection;
use crate::network::handlers::connection_handler::ConnectionHandler;
use crate::network::packet_handler::PacketHandler;

use crab_protocol::buf::{get_var_int_from_stream, ReadByteBuf};
use crab_protocol::protocol::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

pub struct TcpServer {
    address: String,
    pub connections: Arc<Mutex<Vec<Arc<Mutex<Connection>>>>>,
}

impl TcpServer {
    pub fn new(address: String) -> TcpServer {
        TcpServer {
            address,
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();

        loop {
            match listener.accept().await {
                Ok((stream, address)) => {
                    debug!("Connection established {address}!");

                    stream.set_nodelay(true).unwrap();
                    let (read_half, write_half) = tokio::io::split(stream);

                    let connection = Arc::new(Mutex::new(Connection {
                        write_half,
                        state: ProtocolState::Handshake,
                        is_closed: false,
                        address,
                        keepalive_pending: false,
                        last_keepalive_time: Local::now().timestamp_millis(),
                    }));
                    self.connections.lock().await.push(connection.clone());
                    let connections_mutex = Arc::clone(&self.connections);

                    tokio::spawn(async move {
                        TcpServer::handle_connection(connection, read_half).await;
                        connections_mutex
                            .lock()
                            .await
                            .retain(|conn| !conn.try_lock().unwrap().is_closed)
                    });
                }
                Err(err) => {
                    error!("Error accepting connection: {err}");
                }
            }
        }
    }

    pub async fn tick(&self) {
        let connections = self.connections.lock().await;
        for conn in connections.iter() {
            let mut conn = conn.lock().await;
            conn.keep_connection_alive().await;
        }
    }

    async fn handle_connection(connection: Arc<Mutex<Connection>>, mut read_half: ReadHalf<TcpStream>) {
        let connection = Arc::clone(&connection);
        let mut handlers: Vec<Box<dyn PacketHandler + Send>> = vec![Box::new(ConnectionHandler)];

        while let Ok(len) = get_var_int_from_stream(&mut read_half).await {
            let mut buffer = vec![0; len as usize];

            if let Ok(0) = read_half.read_exact(&mut buffer).await {
                break;
            }

            let mut_bytes = BytesMut::from(buffer.as_slice());
            let mut byte = mut_bytes.clone().freeze();

            let packet_id = byte.get_var_int().unwrap();
            let raw_packet = RawPacket::new(packet_id, byte.clone());

            let mut connection = connection.lock().await;
            for handler in &mut handlers {
                match handler.handle_packet(&mut connection, raw_packet.clone()).await {
                    Ok(_) => {}
                    Err(_) => {
                        connection.disconnect("Unexpected error!").await;
                        return;
                    }
                }
            }
        }

        let mut connection = connection.lock().await;
        connection.is_closed = true;
        debug!("Connection closed {:?}!", connection.address);
    }
}
