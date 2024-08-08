use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use crate::game::{Game, GamePhase};
use crate::util::{broadcast_message, ServerMessage, ClientMessage};
use std::thread;
use std::time::Duration;
use crossbeam::thread::scope;

pub fn run() {
    // Création du serveur TCP écoutant sur le port 7878
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    println!("Server listening on port 7878");

    // Initialisation du jeu partagé entre threads avec Arc et Mutex
    let game = Arc::new(Mutex::new(Game::new()));
    
    // Liste des clients connectés partagée entre threads
    let clients = Arc::new(Mutex::new(Vec::new()));

    // Utilisation de "scope" pour créer une boucle d'écoute des clients tout en gérant le multithreading
    scope(|s| {
        // Thread gérant la phase de vote et le lancement du jeu
        s.spawn(|_| {
            let game = Arc::clone(&game); // Clonage des références pour utilisation dans ce thread
            let clients = Arc::clone(&clients);
            let mut countdown = 20; // Compte à rebours de 20 secondes pour la phase de vote

            loop {
                {
                    let mut game = game.lock().unwrap();
                    if game.phase == GamePhase::Voting {
                        // Si le compte à rebours est terminé, démarre le jeu avec la difficulté choisie
                        if countdown == 0 {
                            game.determine_difficulty(); // Détermine la difficulté basée sur les votes
                            broadcast_message(ServerMessage::GameStart(game.difficulty), &clients.lock().unwrap());
                            game.start_game_phase(); // Passe à la phase de jeu
                            countdown = 20; // Réinitialise le compte à rebours pour la prochaine partie
                        } else {
                            println!("Countdown: {}", countdown); // Affiche le temps restant
                            countdown -= 1;
                        }
                    }
                }
                // Attendre 1 seconde avant de décrémenter le compte à rebours
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Boucle pour accepter les nouvelles connexions des clients
        for stream in listener.incoming() {
            let stream = stream.expect("failed to accept connection");
            let game = Arc::clone(&game);
            let clients = Arc::clone(&clients);

            // Pour chaque client connecté, un nouveau thread est créé pour gérer la communication
            s.spawn(|_| {
                println!("New client connected");
                handle_client(stream, game, clients); // Gère la communication avec ce client spécifique
            });
        }
    }).expect("Thread pool failed"); // Gestion d'erreurs si le pool de threads échoue
}

pub fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap()); // Permet de lire les données du client

    {
        let mut clients = clients.lock().unwrap();
        clients.push(stream.try_clone().unwrap()); // Ajoute le nouveau client à la liste des clients
    }

    // Demande au client d'entrer son nom
    write!(stream, "Enter your name: ").unwrap();
    stream.flush().unwrap();

    let mut player_name = String::new();
    reader.read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string(); // Nettoie le nom du joueur

    {
        let mut game = game.lock().unwrap();
        game.add_player(player_name.clone()); // Ajoute le joueur à la partie
        if game.phase == GamePhase::Identification {
            game.start_voting_phase(); // Démarre la phase de vote si on est en phase d'identification
        }
    }

    println!("Player {} has joined the game", player_name);

    loop {
        let mut buffer = vec![];
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) => {
                // Si la connexion est fermée par le client
                println!("Player {} disconnected", player_name);
                break;
            },
            Ok(_) => {
                buffer.pop(); // Retire le délimiteur '\n'
                let message: ClientMessage = match bincode::deserialize(&buffer) {
                    Ok(msg) => msg, // Désérialise le message du client
                    Err(_) => {
                        eprintln!("Failed to deserialize");
                        continue;
                    }
                };
                println!("Received message from {}: {:?}", player_name, message);

                match message {
                    // Gestion des messages de type Guess
                    ClientMessage::Guess(guess) => {
                        let mut game = game.lock().unwrap();
                        if game.phase != GamePhase::Playing {
                            // Si le jeu n'est pas en cours, l'utilisateur ne peut pas deviner
                            write!(stream, "You cannot guess right now. Please wait for the game to start.\n").unwrap();
                            continue;
                        }
                        let result = game.guess(&player_name, guess);
                        let response = match result {
                            Ok(hint) => {
                                // Envoie un indice au client concernant sa devinette
                                println!("Sending hint to {}: {}", player_name, hint);
                                let hint_message = ServerMessage::Hint(hint.to_string());
                                let encoded: Vec<u8> = bincode::serialize(&hint_message).expect("Failed to serialize");
                                stream.write_all(&encoded).expect("Failed to write to client");
                                stream.write_all(b"\n").expect("Failed to write delimiter");
                                
                                if hint == "Vous avez gagné!" {
                                    // Si le joueur a gagné, informe tous les clients et termine le jeu
                                    broadcast_message(ServerMessage::PlayerWon(player_name.clone()), &clients.lock().unwrap());
                                    broadcast_message(ServerMessage::GameEnd, &clients.lock().unwrap());
                                    game.end_game();
                                }
                                ServerMessage::Guess(guess, player_name.clone())
                            },
                            Err(_) => ServerMessage::Guess(guess, player_name.clone()),
                        };
                        // Diffuse la devinette aux autres clients
                        broadcast_message(response, &clients.lock().unwrap());
                    },
                    // Gestion des votes de difficulté
                    ClientMessage::DifficultyVote(difficulty) => {
                        let mut game = game.lock().unwrap();
                        if game.phase != GamePhase::Voting {
                            // Si la phase de vote est terminée, les votes ne sont plus acceptés
                            write!(stream, "Voting phase is over. Please wait for the next game.\n").unwrap();
                            continue;
                        }
                        game.vote_difficulty(difficulty); // Enregistre le vote de difficulté
                        let response = ServerMessage::DifficultyVote(difficulty);
                        broadcast_message(response, &clients.lock().unwrap()); // Diffuse le vote aux autres clients
                    },
                    // Gestion des requêtes pour obtenir la liste des joueurs
                    ClientMessage::RequestPlayers => {
                        let game = game.lock().unwrap();
                        let players: Vec<String> = game.players.keys().cloned().collect();
                        let response = ServerMessage::PlayerList(players);
                        let encoded: Vec<u8> = bincode::serialize(&response).expect("Failed to serialize");
                        stream.write_all(&encoded).expect("Failed to write to client");
                        stream.write_all(b"\n").expect("Failed to write delimiter");
                    },
                    ClientMessage::Join(_) => {}, // Si un client envoie un autre message de type Join, il est ignoré
                }
            },
            Err(_) => {
                eprintln!("Error reading from client.");
                break;
            }
        }
    }

    {
        // Supprime le client de la liste des clients actifs à la déconnexion
        let mut clients = clients.lock().unwrap();
        clients.retain(|client| client.peer_addr().unwrap() != stream.peer_addr().unwrap());
    }

    {
        // Retire le joueur du jeu à sa déconnexion
        let mut game = game.lock().unwrap();
        game.remove_player(&player_name);
    }
}
