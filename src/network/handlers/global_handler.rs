use std::fs::{create_dir_all, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use tokio::net::TcpStream;

use crate::network::packet_handler::PacketHandler;

pub struct GlobalHandler;

impl PacketHandler for GlobalHandler {
    fn handle_packet(&self, _stream: &mut TcpStream, bytes: Bytes, _packet_id: i32) {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        create_dir_all("logs").expect("Couldn't create directory!");

        let mut file = File::create(format!("logs/{}.bin", &time_now.to_string())).unwrap();

        file.write_all(&bytes).unwrap();
    }
}
