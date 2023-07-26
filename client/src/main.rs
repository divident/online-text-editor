use std::{net::TcpStream, io::Write};
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


fn main() {

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        println!("Connnected");

        let data_to_send = Package{
            command: CommandType::Block, 
            payload: [1; 32],
        };

        let serialized_data: Vec<u8> = bincode::serialize(&data_to_send).unwrap();

        // Send the serialized data to the server
        stream.write_all(&serialized_data);

    } else {
        println!("Connection refused!");
    }
}
