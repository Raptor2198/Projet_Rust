use std::net::TcpStream;
use std::io::{self, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use bincode;
mod game;
mod util;
use util::{ServerMessage, ClientMessage};

fn main() {
    // Connexion au serveur via TCP
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to server");
    let mut reader = BufReader::new(stream.try_clone().unwrap()); // Clonage du flux pour lecture.
    let game = Arc::new(Mutex::new(game::Game::new())); // Création d'une instance du jeu protégée par un Mutex pour le partage entre threads.

    // Lecture du nom du joueur à partir de l'entrée standard.
    let mut player_name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut player_name).unwrap();
    player_name = player_name.trim().to_string(); // Suppression des espaces superflus.

    // Envoi d'un message de type `Join` au serveur pour signaler l'entrée du joueur.
    let join_message = ClientMessage::Join(player_name.clone());
    let encoded: Vec<u8> = bincode::serialize(&join_message).expect("Failed to serialize");
    stream.write_all(&encoded).expect("Failed to write to server");
    stream.write_all(b"\n").expect("Failed to write delimiter");

    // Prompt pour permettre au joueur de voter pour la difficulté du jeu.
    println!("Enter your vote!! Choose between 'easy', 'medium', 'hard' to vote for difficulty:");

    let game = Arc::clone(&game); // Clonage de l'arc pour le passer au thread.
    let handle = std::thread::spawn(move || {
        loop {
            let mut buffer = vec![];
            match reader.read_until(b'\n', &mut buffer) {
                Ok(0) => break, // Fin de la connexion si le serveur ferme la connexion.
                Ok(_) => {
                    buffer.pop(); // Suppression du délimiteur '\n' reçu du serveur.
                    // Désérialisation du message reçu.
                    let message: ServerMessage = match bincode::deserialize(&buffer) {
                        Ok(msg) => msg,
                        Err(_) => {
                            eprintln!("Failed to deserialize");
                            continue; // Continuer en cas d'erreur de désérialisation.
                        }
                    };

                    // Traitement du message reçu en fonction de son type.
                    match message {
                        ServerMessage::Hint(hint) => {
                            // Affichage de l'indice reçu et demande de la prochaine supposition.
                            println!("Hint: {}", hint);
                            println!(); 
                            println!("Enter your next guess:");
                        },
                        ServerMessage::PlayerWon(winner) => {
                            // Affichage du message de victoire et fin du jeu.
                            println!("{} has won the game!", winner);
                            break;
                        },
                        ServerMessage::GameStart(difficulty) => {
                            // Affichage de la difficulté sélectionnée et préparation au début du jeu.
                            println!("Game started with difficulty: {:?}. Enjoy the game :) and enter your first guess:", difficulty);
                            let mut game = game.lock().unwrap(); 
                            game.set_difficulty(difficulty); // Définition de la difficulté.
                        },
                        ServerMessage::GameEnd => {
                            // Affichage de la fin du jeu et sortie de la boucle.
                            println!("Game ended");
                            break;
                        },
                        _ => {}, // Autres types de messages ignorés.
                    }
                },
                Err(_) => {
                    eprintln!("Error reading from server."); // Gestion des erreurs de lecture.
                    break;
                }
            }
        }
    });

    // Boucle principale pour traiter l'entrée du joueur.
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Lecture de l'entrée utilisateur.
        let input = input.trim(); // Suppression des espaces superflus.

        if let Ok(guess) = input.parse::<u32>() {
            // Si l'entrée est un nombre, on considère qu'il s'agit d'une supposition.
            let message = ClientMessage::Guess(guess);
            let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
            stream.write_all(&encoded).expect("Failed to write to server");
            stream.write_all(b"\n").expect("Failed to write delimiter");
        } else {
            // Si l'entrée est une chaîne de caractères, on considère qu'il s'agit d'un vote pour la difficulté.
            match input {
                "easy" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Easy);
                    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                "medium" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Medium);
                    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                "hard" => {
                    let message = ClientMessage::DifficultyVote(game::Difficulty::Hard);
                    let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
                    stream.write_all(&encoded).expect("Failed to write to server");
                    stream.write_all(b"\n").expect("Failed to write delimiter");
                },
                _ => println!("Invalid input"), // Gestion des entrées invalides.
            }
        }
    }

    handle.join().unwrap(); // Attente de la fin du thread secondaire avant de terminer le programme principal.
}
