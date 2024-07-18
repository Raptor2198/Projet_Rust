use std::net::TcpStream;
use std::io::{self, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use bincode;

mod util;

use util::ClientMessage;
use util::ServerMessage;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to server");

    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    stream.write_all(name.as_bytes()).unwrap();

    let reader = BufReader::new(stream.try_clone().unwrap());
    let reader = Arc::new(Mutex::new(reader));
    let writer = Arc::new(Mutex::new(stream));

    let reader_clone = Arc::clone(&reader);
    std::thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            let mut reader = reader_clone.lock().unwrap();
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    let buffer = buffer.trim();
                    if !buffer.is_empty() {
                        match bincode::deserialize::<ServerMessage>(buffer.as_bytes()) {
                            Ok(msg) => match msg {
                                ServerMessage::Hint(hint) => println!("{}", hint),
                                _ => println!("Received: {:?}", msg),
                            },
                            Err(_) => println!("Failed to deserialize: {}", buffer),
                        }
                    }
                },
                Err(_) => break,
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "players" {
            let request = ClientMessage::RequestPlayers;
            let encoded: Vec<u8> = bincode::serialize(&request).expect("Failed to serialize");
            let mut writer = writer.lock().unwrap();
            writer.write_all(&encoded).expect("Failed to write to server");
            writer.write_all(b"\n").expect("Failed to write delimiter");
        } else {
            match input.parse::<u32>() {
                Ok(guess) => {
                    let guess_message = ClientMessage::Guess(guess);
                    let encoded: Vec<u8> = bincode::serialize(&guess_message).expect("Failed to serialize");
                    let mut writer = writer.lock().unwrap();
                    writer.write_all(&encoded).expect("Failed to write to server");
                    writer.write_all(b"\n").expect("Failed to write delimiter");
                },
                Err(_) => println!("Please type a number!"),
            }
        }
    }
}
