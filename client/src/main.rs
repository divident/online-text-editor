use std::{net::TcpStream, io::{Write, self, Read}};
use bincode;
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

const CHUNK_SIZE: usize = 32;


fn main() {

    let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    loop {
        // Read from stdin into the buffer.
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break, // End of input
            Ok(n) => n,
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        };

        send_chunk(&buffer);
    }

}

fn send_chunk(chunk: &[u8; 32]) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        println!("Connnected");

        let data_to_send = Package{
            command: CommandType::Block, 
            payload: *chunk,
        };

        let serialized_data: Vec<u8> = bincode::serialize(&data_to_send).unwrap();

        // Send the serialized data to the server
        stream.write_all(&serialized_data).unwrap();

    } else {
        println!("Connection refused!");
    }

}