use std::io::ErrorKind;

use bytes::{Bytes, BytesMut};
use log::{debug, error};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::mpsc::{self, error::TrySendError, Receiver, Sender},
};

use super::{packet_handler::PacketHandler, packet_transformer::PacketTransformer, tcp_server::ProtocolState};

pub struct PlayerConnection {
    pub stream: TcpStream,
    pub state: ProtocolState,
    pub handlers: Vec<Box<dyn PacketHandler + Send>>,
    pub receiver: Receiver<ConnectionMsg>,
    pub sender: Sender<ConnectionMsg>,
    transformer: PacketTransformer,
}

impl PlayerConnection {
    pub fn create(stream: TcpStream, handlers: Vec<Box<dyn PacketHandler + Send>>) -> (Self, ConnectionHandle) {
        let (tx, rx) = mpsc::channel(32);

        let handle = ConnectionHandle::new(tx.clone());

        let connection = PlayerConnection {
            stream,
            state: ProtocolState::Handshake,
            handlers,
            receiver: rx,
            sender: tx,
            transformer: PacketTransformer::new(),
        };
        (connection, handle)
    }

    pub async fn listen(&mut self) {
        'main: loop {
            self.stream.readable().await.unwrap();

            let mut buf = vec![0; 4096];

            match self.stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    buf.truncate(n);
                    self.handle_data(BytesMut::from(&buf[..]));
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    error!("Player connection error {}", e);
                    break;
                }
            }

            while let Ok(msg) = self.receiver.try_recv() {
                use ConnectionMsg::*;

                match msg {
                    SendPacket { bytes } => {
                        self.stream.write_all(&bytes).await.unwrap();
                    }
                    Close => {
                        self.stream.shutdown().await.unwrap();
                        self.receiver.close();
                        break 'main;
                    }
                    SetState { state } => self.state = state,
                };
            }
        }
        debug!("Connection closed {:?}!", self.stream.peer_addr().unwrap());
    }

    fn handle_data(&mut self, data: BytesMut) {
        let packets = self.transformer.decode_packets(data.into());
        for packet in packets {
            for handler in &mut self.handlers {
                handler.handle_packet(&mut self.stream, &mut self.state, packet.1.clone(), packet.0);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ConnectionMsg {
    SendPacket { bytes: Bytes },
    SetState { state: ProtocolState },
    Close,
}

#[derive(Clone)]
pub struct ConnectionHandle {
    sender: Sender<ConnectionMsg>,
}

impl ConnectionHandle {
    pub fn new(sender: Sender<ConnectionMsg>) -> Self {
        Self { sender }
    }

    #[allow(dead_code)]
    pub fn send_packet(&self, bytes: Bytes) -> Result<(), TrySendError<ConnectionMsg>> {
        self.sender.try_send(ConnectionMsg::SendPacket { bytes })?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn close(&self) -> Result<(), TrySendError<ConnectionMsg>> {
        self.sender.try_send(ConnectionMsg::Close)?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn set_state(&self, state: ProtocolState) -> Result<(), TrySendError<ConnectionMsg>> {
        self.sender.try_send(ConnectionMsg::SetState { state })?;
        Ok(())
    }
}
