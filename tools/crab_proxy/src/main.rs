use std::sync::Arc;

use bytes::BytesMut;
use clap::Parser;
use crab_protocol::buf::{packet_from_stream, ReadByteBuf, WriteByteBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

    let handle_c2s = async {
        transfer_data("C2S", &args, &mut client_recv, &mut server_send)
            .await
            .unwrap();
    };
    let handle_s2c = async {
        transfer_data("S2C", &args, &mut server_recv, &mut client_send)
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
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(Some(buffer)) = packet_from_stream(receiver).await {
        let id = buffer.clone().get_var_int().unwrap();
        if buffer.len() >= args.maximum_packet_print_size as usize {
            info!("[{direction}] {id:X}: Packet is too big to be printed!");
        } else {
            info!("[{direction}] {id:X}: {buffer:?}");
        }

        let mut full_packet = BytesMut::new();
        full_packet.put_var_int(buffer.len().try_into().unwrap());
        full_packet.extend_from_slice(&buffer);

        sender.write_all(&full_packet).await?;
    }
    Ok(())
}
