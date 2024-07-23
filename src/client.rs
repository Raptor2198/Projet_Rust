use std::net::TcpStream;
use std::io::{self, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use bincode;
mod game;
mod util;
use util::{ServerMessage, ClientMessage};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to server");
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let game = Arc::new(Mutex::new(game::Game::new()));

    let mut player_name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.trim().to_string();

    let join_message = ClientMessage::Join(player_name.clone());
    let encoded: Vec<u8> = bincode::serialize(&join_message).expect("Failed to serialize");
    stream.write_all(&encoded).expect("Failed to write to server");
    stream.write_all(b"\n").expect("Failed to write delimiter");

    // Add the prompt for voting after sending the join message
    println!("Enter your vote!! Choose between 'easy', 'medium', 'hard' to vote for difficulty:");

    let game = Arc::clone(&game);
    let handle = std::thread::spawn(move || {
        loop {
            let mut buffer = vec![];
            match reader.read_until(b'\n', &mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    buffer.pop(); // Supprimez le délimiteur '\n'
                    let message: ServerMessage = match bincode::deserialize(&buffer) {
                        Ok(msg) => msg,
                        Err(_) => {
                            eprintln!("Failed to deserialize");
                            continue;
                        }
                    };

                    match message {
                        ServerMessage::Hint(hint) => {
                            println!("Hint: {}", hint);
                            println!(); // Ajoutez cette ligne pour un saut de ligne supplémentaire
                            println!("Enter your next guess:");
                        },
                        ServerMessage::PlayerWon(winner) => {
                            println!("{} has won the game!", winner);
                            break;
                        },
                        ServerMessage::GameStart(difficulty) => {
                            println!("Game started with difficulty: {:?}. Enjoy the game :) and enter your first guess:", difficulty);
                            let mut game = game.lock().unwrap();
                            game.set_difficulty(difficulty);
                        },
                        ServerMessage::GameEnd => {
                            println!("Game ended");
                            break;
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    eprintln!("Error reading from server.");
                    break;
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Ok(guess) = input.parse::<u32>() {
            let message = ClientMessage::Guess(guess);
            let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
            stream.write_all(&encoded).expect("Failed to write to server");
            stream.write_all(b"\n").expect("Failed to write delimiter");
        } else {
            match input {
                "easy" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Easy);
                    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                "medium" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Medium);
                    let encoded: Vec<u8> = bincode::serialize(& message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                "hard" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Hard);
                    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                _ => println!("Invalid input"),
            }
        }
    }

    handle.join().unwrap();
}
