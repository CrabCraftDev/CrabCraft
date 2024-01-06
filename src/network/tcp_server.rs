use std::{fs::{OpenOptions, self}, io::{BufReader, Read, Write}, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use std::net::{TcpListener, TcpStream};

use super::packet_byte_buffer::{to_var_int, WriteByteBuffer};

use std::fs::create_dir_all;

use chrono::Utc;
use log::debug;

pub struct TcpServer {
    address: String,
    pub thread: Option<JoinHandle<()>>,
}

impl TcpServer {
    pub fn start(address: String) -> Arc<Mutex<TcpServer>> {
        let server = Arc::new(Mutex::new(TcpServer {
            address,
            thread: None,
        }));

        let listener = TcpListener::bind(&server.lock().unwrap().address).unwrap();

        let cloned_server = Arc::clone(&server);
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                debug!(
                    "Connection established {}",
                    stream.local_addr().unwrap().ip()
                );
                cloned_server.lock().unwrap().handle_connection(stream);
            }
        });

        server.lock().unwrap().thread = Some(handle);

        server
    }

    #[allow(dead_code)]
    pub fn send(&self, mut stream: &TcpStream, bytes: Vec<u8>) {
        if stream.write(to_var_int(bytes.len() as i32).as_slice()).err().is_some() || stream.write_all(bytes.as_slice()).err().is_some() {
            println!("Couldn't send packet!")
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let bytes = buf_reader.bytes();

        create_dir_all("logs").expect("Failed to create directories");
        let path = format!("logs/{}.bin", Utc::now().timestamp());
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .expect("Couldn't open file!");

        for byte in bytes {
            file.write_all(&[byte.unwrap()])
                .expect("Couldn't write to file!");
        }
        
        // prepare for server list ping
        let mut response = vec![];
        response.put_var_int(0);
        let status = fs::read_to_string("logs/status.json")
            .expect("Couldn't find logs/status.json");
        response.put_string(status);

        self.send(&stream, response);
    }
}
