use crate::game::Game;
use crate::util::{broadcast_message, handle_server_message};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Join(String),
    Guess(u32),
    VoteDifficulty(Difficulty),
    RequestScores,
}

pub fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    write!(stream, "Entrez votre pseudo: ").unwrap();
    stream.flush().unwrap();

    let mut player_name = String::new();
    reader.read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();

    {
        let mut game = game.lock().unwrap();
        game.add_player(player_name.clone());
    }

    write!(stream, "Bienvenue, {}! Entrez votre supposition: ", player_name).unwrap();
    stream.flush().unwrap();

    loop {
        let mut guess_input = String::new();
        match reader.read_line(&mut guess_input) {
            Ok(_) => {
                let guess: u32 = match guess_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        write!(stream, "Veuillez entrer un nombre valide.\nEntrez votre supposition: ").unwrap();
                        stream.flush().unwrap();
                        continue;
                    }
                };

                let result: String = {
                    let mut game = game.lock().unwrap();
                    match game.guess(&player_name, guess) {
                        Ok(msg) => msg.to_string(),
                        Err(_) => "Erreur lors de la supposition".to_string(),
                    }
                };

                write!(stream, "{}\n", result).unwrap();
                stream.flush().unwrap();

                if result == "Vous avez gagné!" {
                    break;
                }
            }
            Err(_) => {
                eprintln!("Error reading from client.");
                break;
            }
        }
    }

    {
        let mut game = game.lock().unwrap();
        game.update_high_scores();
        let scores = game.get_scores();
        write!(stream, "Scores actuels:\n{}", scores).unwrap();
        stream.flush().unwrap();
    }
}