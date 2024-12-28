use clap::Parser;
use crab_protocol::buf::packet_from_stream;
use crab_protocol::packet_transformer::PacketTransformer;
use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::packets::login::LoginAcknowledgedC2SPacket;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::protocol_state::ProtocolState::{Configuration, Handshake, Login, Status};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};
use tracing::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP of proxy
    #[arg(short, long, default_value = "127.0.0.1:25566")]
    listen_ip: String,

    /// IP of server to connect
    #[arg(short, long, default_value = "127.0.0.1:25565")]
    target_server_ip: String,

    /// Maximum packet size for which content should be printed
    #[arg(short, long, default_value = "1500")]
    maximum_packet_print_size: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_target(false)
        .init();

    let args = Arc::new(Args::parse());

    info!("Starting CrabProxy...");
    info!("Listening on {}", args.listen_ip);
    info!("Forwarding to {}", args.target_server_ip);

    let proxy_server = TcpListener::bind(&args.listen_ip).await?;

    while let Ok((client, _)) = proxy_server.accept().await {
        let args_clone = Arc::clone(&args);
        tokio::spawn(async move {
            handle_client_connection(client, args_clone).await.unwrap();
        });
    }

    Ok(())
}

async fn handle_client_connection(
    mut client_conn: TcpStream,
    args: Arc<Args>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connection established");

    let mut main_server_conn = TcpStream::connect(&args.target_server_ip).await?;
    let (mut client_recv, mut client_send) = client_conn.split();
    let (mut server_recv, mut server_send) = main_server_conn.split();

    let state = Arc::new(RwLock::new(Handshake));
    let state_clone = state.clone();

    let packet_transformer = PacketTransformer::new();
    let packet_transformer_clone = packet_transformer.clone();

    let handle_c2s = async {
        transfer_data(
            "C2S",
            &args,
            &mut client_recv,
            &mut server_send,
            state_clone,
            packet_transformer,
        )
        .await
        .unwrap();
    };
    let handle_s2c = async {
        transfer_data(
            "S2C",
            &args,
            &mut server_recv,
            &mut client_send,
            state,
            packet_transformer_clone,
        )
        .await
        .unwrap();
    };

    select! {
        _ = handle_c2s => {},
        _ = handle_s2c => {}
    }

    info!("Connection closed");
    Ok(())
}

async fn transfer_data(
    direction: &str,
    args: &Args,
    receiver: &mut (impl AsyncReadExt + Unpin),
    sender: &mut (impl AsyncWriteExt + Unpin),
    state: Arc<RwLock<ProtocolState>>,
    mut packet_transformer: PacketTransformer,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(Some(buffer)) = packet_from_stream(receiver).await {
        let decoded_packet = packet_transformer.decode(buffer)?;

        if decoded_packet.data.len() >= args.maximum_packet_print_size as usize {
            info!(
                "[{direction}] {:X}: Packet is too big to be printed!",
                decoded_packet.id
            );
        } else {
            info!("[{direction}] {:X}: {:?}", decoded_packet.id, decoded_packet.data);
        }

        if *state.read().await == Handshake {
            let handshake = decoded_packet.decode::<HandshakeC2SPacket>(Handshake).unwrap();

            *state.write().await = if handshake.next_state == NextState::Login {
                info!("Switch to login");
                Login
            } else {
                info!("Switch to status/transfer");
                Status
            };
        }

        if *state.read().await == Login
            && direction == "C2S"
            && decoded_packet.decode::<LoginAcknowledgedC2SPacket>(Login).is_some()
        {
            *state.write().await = Configuration;
            info!("Switch to configuration")
        }

        packet_transformer
            .encode_and_write(decoded_packet, *state.read().await, sender)
            .await
            .unwrap();
    }
    Ok(())
}
