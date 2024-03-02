use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use chrono::Local;
use log::{debug, error};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio::io::ReadHalf;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task::block_in_place;

use crate::network::connection::Connection;
use crate::network::handlers::connection_handler::ConnectionHandler;
use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::network::packet_handler::PacketHandler;
use crate::utils::network::{get_var_int_from_stream, to_var_int};

pub struct TcpServer {
    address: String,
    pub connections: Arc<Mutex<Vec<Arc<Mutex<Connection>>>>>,
}

#[derive(Debug, PartialEq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
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

    pub async fn send(stream: &mut (impl AsyncWriteExt + Unpin), bytes: Bytes) {
        stream.write_all(&to_var_int(bytes.len() as i32)).await.unwrap();
        stream.write_all(&bytes).await.unwrap();
    }

    pub fn send_sync(connection: &mut Connection, bytes: Bytes) {
        block_in_place(|| Handle::current().block_on(TcpServer::send(&mut connection.write_half, bytes)));
    }

    pub async fn tick(&self) {
        let connections = self.connections.lock().await;
        for conn in connections.iter() {
            let mut conn = conn.lock().await;
            if conn.state == ProtocolState::Play {
                conn.keep_connection_alive().await;
            }
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

            let packet_id = byte.get_var_int();

            let mut connection = connection.lock().await;
            for handler in &mut handlers {
                handler.handle_packet(&mut connection, byte.clone(), packet_id);
            }
        }

        let mut connection = connection.lock().await;
        connection.is_closed = true;
        debug!("Connection closed {:?}!", connection.address);
    }
}
