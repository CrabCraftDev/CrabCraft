use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
};

fn main() {
    println!("CrabCraft server starting...");
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established! {}", stream.local_addr().unwrap().ip());
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let bytes = buf_reader.bytes();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("log/packet.txt").unwrap(); // that unwrap is temporary :)

    for bt in bytes {
        let byte = bt.unwrap();
        file.write(&[byte]).expect("Couldn't log packet!");
        print!("{} ", byte)
    }
}