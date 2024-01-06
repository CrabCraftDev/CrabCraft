mod network;

use network::tcp_server::TcpServer;

fn main() {
    println!("CrabCraft server starting...");
    let server = TcpServer::start("127.0.0.1:25565".to_string());
    while server.lock().unwrap().thread.is_some() {
        // TODO: Continue implementing!
    }
}