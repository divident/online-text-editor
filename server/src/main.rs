use std::sync::{Arc, Mutex};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};

use server::ThreadPool;

#[derive(Debug, Serialize, Deserialize)]
enum CommandType {
    Read,
    Write,
    Block,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    size: u8,
    payload: [u8; 32],
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let buffer = Arc::new(Mutex::new(String::new()));

    let pool = ThreadPool::new(5);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let buff_copy = Arc::clone(&buffer);
        pool.execute(|| {
            handle_connection(stream, buff_copy);
        });
    }
}

fn handle_connection(mut stream: TcpStream, buffer: Arc<Mutex<String>>) {
    let mut buf: [u8; 36] = [0; 36];
    stream.read(&mut buf).unwrap();

    let result: Package = bincode::deserialize(&buf).unwrap();

    println!("New connection from {}", stream.peer_addr().unwrap());
    let size = result.size as usize;
    let received_string = String::from_utf8_lossy(&result.payload[0..size]).into_owned();
    let mut buffer = buffer.lock().unwrap();
    println!("Received string {}", received_string);
    buffer.push_str(&received_string);
    println!("Current buffer {:?}", buffer);
}
