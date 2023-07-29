use bincode;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

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

const CHUNK_SIZE: usize = 32;

fn main() {
    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    loop {
        // Read from stdin into the buffer.
        match io::stdin().read(&mut buffer) {
            Ok(0) => break, // End of input
            Ok(n) => {
                send_chunk(&buffer, n as u8);
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        };
    }
}

fn send_chunk(chunk: &[u8; 32], chunk_size: u8) {
    let data_to_send = Package {
        size: chunk_size,
        payload: *chunk,
    };
    let serialized_data: Vec<u8> = bincode::serialize(&data_to_send).unwrap();

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        // Send the serialized data to the server
        stream.write(&serialized_data).unwrap();
        println!("Connnected");
    } else {
        println!("Connection refused!");
    }
}
