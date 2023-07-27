use std::{net::{TcpListener, TcpStream}, io::Read};

use serde::{Serialize, Deserialize};

use server::ThreadPool;

#[derive(Debug, Serialize, Deserialize)]
enum CommandType {
    Read,
    Write,
    Block
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    command: CommandType,
    payload: [u8; 32]

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    let pool = ThreadPool::new(5);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
           handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
        let mut buf: [u8; 36] = [0; 36];
        stream.read(&mut buf).unwrap();

        let result: Package = bincode::deserialize(&buf).unwrap();
        println!("New connection from {}", stream.peer_addr().unwrap());
        println!("{:?}", String::from_utf8_lossy(&result.payload));
}
