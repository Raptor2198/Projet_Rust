use crate::game::Difficulty;
use std::net::TcpStream;
use std::io::Write; // Ajout de cet import
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    DifficultyVote(crate::game::Difficulty),
    Guess(u32, String),
    PlayerWon(String),
    GameStart(crate::game::Difficulty),
    GameEnd,
    ScoreBoard(String),
    PlayerList(Vec<String>),
    Hint(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Guess(u32),
    DifficultyVote(crate::game::Difficulty),
    RequestPlayers,
}

pub fn broadcast_message<T: Serialize>(message: T, clients: &Vec<TcpStream>) {
    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
    for mut client in clients {
        client.write_all(&encoded).expect("Failed to write to client");
        client.write_all(b"\n").expect("Failed to write delimiter");
    }
}
