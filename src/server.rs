use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use crossbeam::thread::scope;
use crate::game::Game;
use crate::util::{broadcast_message, ServerMessage, ClientMessage};

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    println!("Server listening on port 7878");
    let game = Arc::new(Mutex::new(Game::new()));
    let clients = Arc::new(Mutex::new(Vec::new()));
    
    scope(|s| {
        for stream in listener.incoming() {
            let stream = stream.expect("failed to accept connection");
            let game = Arc::clone(&game);
            let clients = Arc::clone(&clients);
            
            s.spawn(move |_| {
                println!("New client connected");
                handle_client(stream, game, clients);
            });
        }
    }).expect("Thread pool failed");
}

pub fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    // Ajouter le client à la liste des clients
    {
        let mut clients = clients.lock().unwrap();
        clients.push(stream.try_clone().unwrap());
    }

    write!(stream, "Enter your name: ").unwrap();
    stream.flush().unwrap();

    let mut player_name = String::new();
    reader.read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();

    {
        let mut game = game.lock().unwrap();
        game.add_player(player_name.clone());
    }

    println!("Player {} has joined the game", player_name);

    loop {
        let mut buffer = vec![];
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) => {
                println!("Player {} disconnected", player_name);
                break;
            },
            Ok(_) => {
                buffer.pop(); // Supprimez le délimiteur '\n'
                let message: ClientMessage = match bincode::deserialize(&buffer) {
                    Ok(msg) => msg,
                    Err(_) => {
                        eprintln!("Failed to deserialize");
                        continue;
                    }
                };
                println!("Received message from {}: {:?}", player_name, message);
                
                match message {
                    ClientMessage::Guess(guess) => {
                        let mut game = game.lock().unwrap();
                        let result = game.guess(&player_name, guess);
                        let response = match result {
                            Ok(hint) => {
                                broadcast_message(ServerMessage::Hint(hint.to_string()), &clients.lock().unwrap());
                                if hint == "Vous avez gagné!" {
                                    ServerMessage::PlayerWon(player_name.clone())
                                } else {
                                    ServerMessage::Guess(guess, player_name.clone())
                                }
                            },
                            Err(_) => ServerMessage::Guess(guess, player_name.clone()),
                        };
                        broadcast_message(response, &clients.lock().unwrap());
                    },
                    ClientMessage::DifficultyVote(difficulty) => {
                        let mut game = game.lock().unwrap();
                        game.set_difficulty(difficulty);
                        let response = ServerMessage::DifficultyVote(difficulty);
                        broadcast_message(response, &clients.lock().unwrap());
                    },
                    ClientMessage::RequestPlayers => {
                        let game = game.lock().unwrap();
                        let players: Vec<String> = game.players.keys().cloned().collect();
                        let response = ServerMessage::PlayerList(players);
                        let encoded: Vec<u8> = bincode::serialize(&response).expect("Failed to serialize");
                        stream.write_all(&encoded).expect("Failed to write to client");
                        stream.write_all(b"\n").expect("Failed to write delimiter");
                    },
                }
            },
            Err(_) => {
                eprintln!("Error reading from client.");
                break;
            }
        }
    }

    // Retirer le client de la liste des clients
    {
        let mut clients = clients.lock().unwrap();
        clients.retain(|client| client.peer_addr().unwrap() != stream.peer_addr().unwrap());
    }

    println!("Player {} disconnected", player_name);
}
