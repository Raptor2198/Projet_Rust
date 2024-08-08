use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::Write;
use crate::game::Difficulty;

// `ServerMessage` définit les différents types de messages que le serveur peut envoyer aux clients.
// Chaque variant correspond à un type de message particulier avec des données spécifiques associées.
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    TimerStart(u32),                // Démarrage du minuteur avec un temps donné (en secondes).
    GameStart(Difficulty),           // Indique que le jeu commence avec un niveau de difficulté spécifique.
    Hint(String),                    // Envoie un indice sous forme de chaîne de caractères.
    PlayerWon(String),               // Informe que le joueur avec le nom donné a gagné.
    GameEnd,                         // Signale la fin du jeu.
    Guess(u32, String),              // Informe les clients d'une tentative de devinette par un joueur (valeur de la devinette et nom du joueur).
    DifficultyVote(Difficulty),      // Envoie un vote pour la difficulté du jeu.
    PlayerList(Vec<String>),         // Envoie la liste des joueurs connectés.
}

// `ClientMessage` définit les différents types de messages que le client peut envoyer au serveur.
// Chaque variant correspond à un type de message particulier avec des données spécifiques associées.
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Guess(u32),                      // Envoie une devinette avec une valeur numérique.
    DifficultyVote(Difficulty),      // Envoie un vote pour une difficulté spécifique.
    Join(String),                    // Requête pour rejoindre le jeu avec un nom de joueur.
    RequestPlayers,                  // Demande la liste des joueurs actuellement connectés.
}

    // `broadcast_message` envoie un message à tous les clients connectés.
    // Le message est sérialisé en une séquence d'octets avant d'être envoyé.
pub fn broadcast_message(message: ServerMessage, clients: &Vec<TcpStream>) {
    for mut client in clients.iter() {
        let encoded: Vec<u8> = bincode::serialize(&message).expect("Failed to serialize");
        client.write_all(&encoded).expect("Failed to write to client");
        client.write_all(b"\n").expect("Failed to write delimiter");
    }
}
