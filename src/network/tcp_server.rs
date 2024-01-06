use std::{fs::OpenOptions, io::{BufReader, Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

pub struct TcpServer {
    address: String,
    pub thread: Option<JoinHandle<()>>,
}

impl TcpServer {
    pub fn start(addr: String) -> Arc<Mutex<TcpServer>> {
        let server = Arc::new(Mutex::new(TcpServer {
            address: addr,
            thread: None,
        }));

        let server_clone = Arc::clone(&server);
        let thread = thread::spawn(move || {
            let server = server_clone.lock().unwrap();
            server.run();
        });
        server.lock().unwrap().thread = Some(thread);
        server
    }

    fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established! {}", stream.local_addr().unwrap().ip());
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let bytes = buf_reader.bytes();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("log/packet.bin").expect("Couldn't log packet!");

        for bt in bytes {
            let byte = bt.unwrap();
            file.write_all(&[byte]).expect("Couldn't log packet!");
            print!("{} ", byte)
        }
    }
}