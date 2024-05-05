use bytes::BytesMut;
use clap::Parser;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

use crab_protocol::buf::{get_var_int_from_stream, to_var_int, ReadByteBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP of proxy
    #[arg(short, long, default_value = "127.0.0.1:25566")]
    listen: String,

    /// IP of server to connect
    #[arg(short, long, default_value = "127.0.0.1:25565")]
    target: String,

    /// Maximum packet size for which content should be printed
    #[arg(short, long, default_value = "1500")]
    print_threshold: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let args_arc = Arc::new(args);
    println!("CrabCraft proxy started...");

    let proxy_server = TcpListener::bind(&args_arc.listen).await?;

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

    let mut main_server_conn = TcpStream::connect(&args.target).await?; // Replace with your target server
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
    while let Ok(len) = get_var_int_from_stream(receiver).await {
        let mut buffer = vec![0; len as usize];
        if let Ok(0) = receiver.read_exact(&mut buffer).await {
            break;
        }

        let id = buffer.clone().as_slice().get_var_int().unwrap();

        if buffer.len() >= args.print_threshold as usize {
            println!("[{direction}] {id:X}: Packet is too big to be printed!");
        } else {
            let buffer_str = BytesMut::from(buffer.as_slice()).freeze();
            println!("[{direction}] {id:X}: {buffer_str:?}");
        }

        let mut full_packet = BytesMut::from_iter(to_var_int(len));
        full_packet.extend_from_slice(&buffer);

        sender.write_all(&full_packet).await?;
    }
    Ok(())
}
