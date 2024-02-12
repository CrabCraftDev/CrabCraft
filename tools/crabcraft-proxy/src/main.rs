use bytes::BytesMut;
use clap::Parser;
use tokio::{
    net::{TcpListener, TcpStream},
    select,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crab_craft::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crab_craft::utils::network::{get_var_int_from_stream, to_var_int};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP of proxy
    #[arg(short, long, default_value = "127.0.0.1:25566")]
    client: String,

    /// IP of server to connect
    #[arg(short, long, default_value = "127.0.0.1:25565")]
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("CrabCraft proxy started...");

    let proxy_server = TcpListener::bind(args.client).await?;

    while let Ok((client, _)) = proxy_server.accept().await {
        let server_target = args.target.clone();
        tokio::spawn(async move {
            handle_client_conn(client, server_target).await.unwrap();
        });
    }

    Ok(())
}

async fn handle_client_conn(mut client_conn: TcpStream, server: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connection established!");

    let mut main_server_conn = TcpStream::connect(server).await?; // Replace with your target server
    let (mut client_recv, mut client_send) = client_conn.split();
    let (mut server_recv, mut server_send) = main_server_conn.split();

    let handle_one = async {
        transfer_data("C2S", &mut client_recv, &mut server_send).await.unwrap();
    };
    let handle_two = async {
        transfer_data("S2C", &mut server_recv, &mut client_send).await.unwrap();
    };

    select! {
        _ = handle_one => {},
        _ = handle_two => {}
    }

    println!("Connection closed!");
    Ok(())
}

async fn transfer_data(
    direction: &str,
    receiver: &mut (impl AsyncReadExt + Unpin),
    sender: &mut (impl AsyncWriteExt + Unpin),
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(len) = get_var_int_from_stream(receiver).await {
        let mut buffer = vec![0; len as usize];
        if let Ok(0) = receiver.read_exact(&mut buffer).await {
            break;
        }

        let mut buffer_copy = buffer.as_slice();
        let id = buffer_copy.get_var_int();
        let bytes_str = BytesMut::from(&buffer as &[u8]).freeze();
        println!("[{direction}] {id:X}: {bytes_str:?}");

        let prefix = to_var_int(len);
        let full_packet = &[prefix, buffer].concat();

        sender.write_all(full_packet).await?;
    }
    Ok(())
}
