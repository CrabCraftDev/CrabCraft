use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::{stream::FuturesUnordered, StreamExt};
use kanal::Sender;
use log::{debug, error};

use tokio::{
    net::{TcpListener, TcpStream},
    select,
    time::{self, MissedTickBehavior},
};

use crate::network::connection::NetworkConnection;

const TCP_SEVER_TPS: u64 = 1;

pub struct TcpServer {
    address: String,
    pub connections: Arc<Mutex<Vec<NetworkConnection>>>,
    new_connection_sender: Sender<NetworkConnection>,
}

impl TcpServer {
    pub fn new(address: String, new_connection_sender: Sender<NetworkConnection>) -> TcpServer {
        TcpServer {
            address,
            connections: Arc::new(Mutex::new(Vec::new())),
            new_connection_sender,
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();

        let mut interval = time::interval(Duration::from_millis(1000 / TCP_SEVER_TPS));
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

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
                _ = interval.tick() => {
                    self.tick().await;
                },
                Some(()) = connections.next(), if !connections.is_empty() => {},
            }
        }
    }

    pub async fn accept_connection(&self, stream: TcpStream, address: SocketAddr) {
        debug!("Connection established {}!", address);

        let connection = NetworkConnection::new(self.new_connection_sender.clone());
        // Additional scope so the lock gets released before await
        {
            let mut connections = self.connections.lock().unwrap();
            connections.push(connection.clone());
        }
        connection.accept(stream).await;

        debug!("Connection closed {}!", address);

        let mut connections = self.connections.lock().unwrap();
        connections.retain(|connection| !connection.incoming_packets_sender.is_closed());
    }

    pub async fn tick(&self) {
        for connection in self.connections.lock().unwrap().iter() {
            connection.keep_connection_alive();
        }
    }
}
