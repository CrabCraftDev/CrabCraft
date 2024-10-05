use std::net::SocketAddr;

use futures::{stream::FuturesUnordered, StreamExt};
use kanal::Sender;
use tracing::{debug, error, info};

use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

use crate::{entity::player::network::incoming_connection::IncomingConnection, network::connection::NetworkConnection};

pub struct TcpServer {
    address: String,
    /// After player connects and finishes handshake and most of login state
    /// Connection is sent through this channel, to be added to ECS
    incoming_connection_sender: Sender<IncomingConnection>,
}

impl TcpServer {
    pub fn new(address: String, incoming_connection_sender: Sender<IncomingConnection>) -> TcpServer {
        TcpServer {
            address,
            incoming_connection_sender,
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        info!("Listening on {}", self.address);

        let mut connections = FuturesUnordered::new();

        loop {
            select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, address)) => {
                            stream.set_nodelay(true).unwrap();
                            connections.push(self.accept_connection(stream, address));
                        }
                        Err(err) => {
                            error!("Error accepting connection: {err}");
                        }
                    }
                },
                Some(()) = connections.next(), if !connections.is_empty() => {},
            }
        }
    }

    pub async fn accept_connection(&self, stream: TcpStream, address: SocketAddr) {
        debug!("Connection established {address}!");

        let connection = NetworkConnection::new(self.incoming_connection_sender.clone());

        connection.accept(stream).await;

        debug!("Connection closed {address}!");
    }
}
