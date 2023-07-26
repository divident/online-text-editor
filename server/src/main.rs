use std::{net::TcpListener, io::Read};

use serde::{Serialize, Deserialize};

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

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf: [u8; 36] = [0; 36];
        stream.read(&mut buf).unwrap();

        let result: Package = bincode::deserialize(&buf).unwrap();
        println!("New connection from {}", stream.peer_addr().unwrap());
        println!("{:?}", result);
    }
}

