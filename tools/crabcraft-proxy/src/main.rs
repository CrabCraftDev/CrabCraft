use std::sync::Arc;

use bytes::BytesMut;
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

use crab_protocol::buf::{packet_from_stream, ReadByteBuf, WriteByteBuf};

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
    let args = Args::parse();
    let args_arc = Arc::new(args);
    println!("CrabCraft proxy started...");

    let proxy_server = TcpListener::bind(&args_arc.listen_ip).await?;

    while let Ok((client, _)) = proxy_server.accept().await {
        let args_clone = Arc::clone(&args_arc);
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
    println!("Connection established!");

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

    println!("Connection closed!");
    Ok(())
}

async fn transfer_data(
    direction: &str,
    args: &Args,
    receiver: &mut (impl AsyncReadExt + Unpin),
    sender: &mut (impl AsyncWriteExt + Unpin),
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(Some(mut buffer)) = packet_from_stream(receiver).await {
        let id = buffer.get_var_int().unwrap();

        if buffer.len() >= args.maximum_packet_print_size as usize {
            println!("[{direction}] {id:X}: Packet is too big to be printed!");
        } else {
            println!("[{direction}] {id:X}: {buffer:?}");
        }

        let mut full_packet = BytesMut::new();
        full_packet.put_var_int(buffer.len().try_into().unwrap());
        full_packet.extend_from_slice(&buffer);

        sender.write_all(&full_packet).await?;
    }
    Ok(())
}
