use std::{
    fs::OpenOptions,
    io::{BufReader, Read, Write},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
use std::net::{TcpListener, TcpStream};

use chrono::Utc;
use log::debug;

use crate::utils::fs::create_subdirectories;

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
    pub fn send(&self, mut stream: TcpStream, bytes: &[u8]) {
        stream.write_all(bytes).unwrap();
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let bytes = buf_reader.bytes();

        let path = format!("logs/{}.bin", Utc::now().timestamp());
        create_subdirectories(&path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .expect("Couldn't open file!");

        for byte in bytes {
            file.write_all(&[byte.unwrap()])
                .expect("Couldn't write to file!");
        }
    }
}
