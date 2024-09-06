use crate::print_error_issues_url;
use crab_nbt::NbtTag;
use crab_protocol::packet::Packet;
use crab_protocol::packets::common::DisconnectS2CPacket;
use crab_protocol::packets::login::LoginDisconnectS2CPacket;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::protocol_state::ProtocolState::{Handshake, Status};
use crab_protocol::raw_packet::RawPacket;
use crossbeam::sync::ShardedLock;
use evenio::prelude::Component;
use kanal::{Receiver, Sender};
use log::{error, info, warn};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Clone, Component)]
pub struct Connection {
    // State is accessed from connection and main thread
    state: Arc<ShardedLock<ProtocolState>>,
    pub address: SocketAddr,
    pub outgoing_packets_sender: Sender<RawPacket>,
    pub incoming_packets_receiver: Receiver<RawPacket>,
}

impl Connection {
    pub fn new(
        address: SocketAddr,
        outgoing_packets_sender: Sender<RawPacket>,
        incoming_packets_receiver: Receiver<RawPacket>,
    ) -> Self {
        Self {
            state: Arc::new(ShardedLock::new(Handshake)),
            address,
            outgoing_packets_sender,
            incoming_packets_receiver,
        }
    }

    pub fn send_packet(&self, packet: impl Packet) {
        if self.is_closed() {
            error!("Tried to send packet while connection closed!");
            print_error_issues_url();
            return;
        }

        match RawPacket::from_packet(packet, self.state()) {
            Ok(raw_packet) => self.outgoing_packets_sender.send(raw_packet).unwrap_or(()),
            Err(err) => error!("Serialization error: {}", err),
        };
    }

    pub fn disconnect(&mut self, reason: &str) {
        if self.is_closed() {
            warn!("Connection::disconnect() called twice!");
            print_error_issues_url();
            return;
        }

        let reason = reason.to_string();
        let state = self.state();
        match state {
            ProtocolState::Configuration | ProtocolState::Play => {
                self.send_packet(DisconnectS2CPacket {
                    reason: NbtTag::String(reason.clone()).into(),
                });
            }
            ProtocolState::Login => {
                self.send_packet(LoginDisconnectS2CPacket {
                    reason: json!({"text": reason}).to_string().into(),
                });
            }
            _ => {}
        }

        self.close();
    }

    pub fn close(&mut self) {
        self.incoming_packets_receiver.close();
        self.outgoing_packets_sender.close();
        if self.state() != Status {
            info!("Disconnecting {:?}", self.address);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.incoming_packets_receiver.is_closed() || self.outgoing_packets_sender.is_closed()
    }

    pub fn state(&self) -> ProtocolState {
        *self.state.read().unwrap()
    }

    pub fn set_state(&self, new_state: ProtocolState) {
        *self.state.write().unwrap() = new_state;
    }
}
