use futures::{stream::FuturesUnordered, StreamExt};
use kanal::AsyncSender;
use std::net::SocketAddr;
use tracing::{debug, error, info};

use crate::events::asynchronous::AsyncEventHolder;
use crate::network::connection::NetworkConnection;
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

pub struct TcpServer {
    address: String,
    /// Used to communicate with the main thread
    event_sender: AsyncSender<AsyncEventHolder>,
}

impl TcpServer {
    pub fn new(address: String, event_sender: AsyncSender<AsyncEventHolder>) -> TcpServer {
        TcpServer { address, event_sender }
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

        let connection = NetworkConnection::new(address.ip(), self.event_sender.clone());
        connection.accept(stream).await;

        debug!("Connection closed {address}!");
    }
}
