use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::Write;
use crate::game::Difficulty;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    TimerStart(u32),
    GameStart(Difficulty),
    Hint(String),
    PlayerWon(String),
    GameEnd,
    Guess(u32, String),
    DifficultyVote(Difficulty),
    PlayerList(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Guess(u32),
    DifficultyVote(Difficulty),
    Join(String),
    RequestPlayers,
}

pub fn broadcast_message(message: ServerMessage, clients: &Vec<TcpStream>) {
    for mut client in clients.iter() {
        let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
        client.write_all(&encoded).expect("Failed to write to client");
        client.write_all(b"\n").expect("Failed to write delimiter");
    }
}
