// server.rs
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use crate::game::{Game, GamePhase};
use crate::util::{broadcast_message, ServerMessage, ClientMessage};
use std::thread;
use std::time::Duration;
use crossbeam::thread::scope;

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    println!("Server listening on port 7878");
    let game = Arc::new(Mutex::new(Game::new()));
    let clients = Arc::new(Mutex::new(Vec::new()));

    scope(|s| {
        s.spawn(|_| {
            let game = Arc::clone(&game);
            let clients = Arc::clone(&clients);
            let mut countdown = 20;

            loop {
                {
                    let mut game = game.lock().unwrap();
                    if game.phase == GamePhase::Voting {
                        if countdown == 0 {
                            game.determine_difficulty();
                            broadcast_message(ServerMessage::GameStart(game.difficulty), &clients.lock().unwrap());
                            game.start_game_phase();
                            countdown = 20;
                        } else {
                            println!("Countdown: {}", countdown); // Timer check
                            countdown -= 1;
                        }
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        for stream in listener.incoming() {
            let stream = stream.expect("failed to accept connection");
            let game = Arc::clone(&game);
            let clients = Arc::clone(&clients);

            s.spawn(|_| {
                println!("New client connected");
                handle_client(stream, game, clients);
            });
        }
    }).expect("Thread pool failed");
}

pub fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

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
        if game.phase == GamePhase::Identification {
            game.start_voting_phase();
        }
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
                buffer.pop();
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
                        if game.phase != GamePhase::Playing {
                            write!(stream, "You cannot guess right now. Please wait for the game to start.\n").unwrap();
                            continue;
                        }
                        let result = game.guess(&player_name, guess);
                        let response = match result {
                            Ok(hint) => {
                                // Envoyer l'indice uniquement au client qui fait la devinette
                                println!("Sending hint to {}: {}", player_name, hint);
                                let hint_message = ServerMessage::Hint(hint.to_string());
                                let encoded: Vec<u8> = bincode::serialize(&hint_message).expect("Failed to serialize");
                                stream.write_all(&encoded).expect("Failed to write to client");
                                stream.write_all(b"\n").expect("Failed to write delimiter");
                                
                                if hint == "Vous avez gagné!" {
                                    broadcast_message(ServerMessage::PlayerWon(player_name.clone()), &clients.lock().unwrap());
                                    broadcast_message(ServerMessage::GameEnd, &clients.lock().unwrap());
                                    game.end_game();
                                }
                                ServerMessage::Guess(guess, player_name.clone())
                            },
                            Err(_) => ServerMessage::Guess(guess, player_name.clone()),
                        };
                        // Informer les autres clients de la devinette
                        broadcast_message(response, &clients.lock().unwrap());
                    },
                    ClientMessage::DifficultyVote(difficulty) => {
                        let mut game = game.lock().unwrap();
                        if game.phase != GamePhase::Voting {
                            write!(stream, "Voting phase is over. Please wait for the next game.\n").unwrap();
                            continue;
                        }
                        game.vote_difficulty(difficulty);
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
                    ClientMessage::Join(_) => {},
                }
            },
            Err(_) => {
                eprintln!("Error reading from client.");
                break;
            }
        }
    }

    {
        let mut clients = clients.lock().unwrap();
        clients.retain(|client| client.peer_addr().unwrap() != stream.peer_addr().unwrap());
    }

    {
        let mut game = game.lock().unwrap();
        game.remove_player(&player_name);
    }

}
