use std::sync::Arc;

use bytes::BytesMut;
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

use crab_protocol::{
    buf::{ReadByteBuf, WriteByteBuf, CONTINUE_BIT, SEGMENT_BITS},
    error::ProtocolError,
};

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
    while let Ok(len) = get_var_int_from_stream(receiver).await {
        let mut buffer = vec![0; len as usize];
        if let Ok(0) = receiver.read_exact(&mut buffer).await {
            break;
        }

        let id = buffer.clone().as_slice().get_var_int().unwrap();

        if buffer.len() >= args.maximum_packet_print_size as usize {
            println!("[{direction}] {id:X}: Packet is too big to be printed!");
        } else {
            let buffer_str = BytesMut::from(buffer.as_slice()).freeze();
            println!("[{direction}] {id:X}: {buffer_str:?}");
        }

        let mut full_packet = BytesMut::new();
        full_packet.put_var_int(len);
        full_packet.extend_from_slice(&buffer);

        sender.write_all(&full_packet).await?;
    }
    Ok(())
}

pub async fn get_var_int_from_stream(stream: &mut (impl AsyncReadExt + Unpin)) -> Result<i32, ProtocolError> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;

    loop {
        let current_byte = stream.read_i8().await?;

        value |= (current_byte as i32 & SEGMENT_BITS) << position;
        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;
        if position >= 32 {
            return Err(ProtocolError::TooBigVarInt(32, position + 1));
        }
    }

    Ok(value)
}
