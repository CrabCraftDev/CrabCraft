use network::tcp_server::TcpServer;

mod network;

fn main() {
    println!("CrabCraft server starting...");

    let server = TcpServer::start("127.0.0.1:25565".to_string());
    let handle = server.lock().unwrap().thread.take().expect("No thread handle");

    handle.join().expect("Server had unexpected error!!");
    println!("CrabCraft server disabled!")
}