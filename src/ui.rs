use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::game::Game;

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
                        Ok(message) => message.to_string(),
                        Err(message) => message.to_string(),
                    }
                };

                write!(stream, "{}\nEntrez une nouvelle supposition: ", result).unwrap();
                stream.flush().unwrap();

                if result == "Vous avez gagné!" {
                    break;
                }
            }
            Err(_) => {
                write!(stream, "Erreur de lecture de l'entrée.").unwrap();
                stream.flush().unwrap();
                break;
            }
        }
    }
}
