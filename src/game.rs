use std::collections::HashMap;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::time::{Instant, Duration};

// Enumération représentant les niveaux de difficulté possibles du jeu.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

// Structure représentant l'état du jeu.
pub struct Game {
    pub players: HashMap<String, u32>, // Liste des joueurs et leurs scores respectifs.
    secret_number: u32, // Nombre secret que les joueurs doivent deviner.
    pub difficulty: Difficulty, // Niveau de difficulté actuel du jeu.
    high_scores: Vec<(String, u32)>, // Liste des meilleurs scores.
    difficulty_votes: HashMap<Difficulty, u32>, // Votes pour déterminer la difficulté.
    start_time: Option<Instant>, // Instant où le jeu a commencé (utilisé pour gérer le timing des phases).
    pub phase: GamePhase, // Phase actuelle du jeu.
}

// Enumération représentant les différentes phases du jeu.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum GamePhase {
    Identification, // Phase où les joueurs rejoignent le jeu.
    Voting, // Phase où les joueurs votent pour la difficulté.
    Playing, // Phase où les joueurs jouent (devinent le nombre secret).
}

impl Game {
    // Fonction de création d'un nouvel état de jeu.
    pub fn new() -> Game {
        Game {
            players: HashMap::new(), // Initialise la liste des joueurs.
            secret_number: rand::thread_rng().gen_range(1..101), // Génère un nombre secret aléatoire entre 1 et 100.
            difficulty: Difficulty::Easy, // Définit la difficulté par défaut à "Facile".
            high_scores: vec![], // Initialise la liste des meilleurs scores.
            difficulty_votes: HashMap::new(), // Initialise la carte des votes de difficulté.
            start_time: None, // Aucun moment de début tant que le jeu n'a pas commencé.
            phase: GamePhase::Identification, // Le jeu commence dans la phase d'identification.
        }
    }

    // Ajoute un joueur au jeu.
    pub fn add_player(&mut self, name: String) {
        self.players.insert(name, 0); // Le score initial du joueur est de 0.
    }

    // Retire un joueur du jeu.
    pub fn remove_player(&mut self, name: &str) {
        self.players.remove(name); // Retire le joueur de la liste des joueurs.
    }
    
    // Gère la tentative de devinette d'un joueur.
    pub fn guess(&mut self, player_name: &str, guess: u32) -> Result<&'static str, ()> {
        if self.phase != GamePhase::Playing { // Les devinettes ne sont acceptées que pendant la phase de jeu.
            return Err(());
        }
        let hint = if guess < self.secret_number { // Si la devinette est inférieure au nombre secret.
            "C’est plus"
        } else if guess > self.secret_number { // Si la devinette est supérieure au nombre secret.
            "C’est moins"
        } else { // Si la devinette est correcte.
            self.players.insert(player_name.to_string(), guess); // Met à jour le score du joueur.
            self.high_scores.push((player_name.to_string(), guess)); // Ajoute le joueur à la liste des meilleurs scores.
            "Vous avez gagné!"
        };
        Ok(hint) // Retourne l'indice (ou le message de victoire).
    }

    // Définit la difficulté du jeu et ajuste la plage du nombre secret en conséquence.
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        match difficulty {
            Difficulty::Easy => self.secret_number = rand::thread_rng().gen_range(1..101), // Facile: 1 à 100.
            Difficulty::Medium => self.secret_number = rand::thread_rng().gen_range(1..501), // Moyen: 1 à 500.
            Difficulty::Hard => self.secret_number = rand::thread_rng().gen_range(1..1001), // Difficile: 1 à 1000.
        }
    }

    // Permet aux joueurs de voter pour la difficulté.
    pub fn vote_difficulty(&mut self, difficulty: Difficulty) {
        let count = self.difficulty_votes.entry(difficulty).or_insert(0); // Incrémente le nombre de votes pour la difficulté choisie.
        *count += 1;
    }

    // Détermine la difficulté finale basée sur les votes des joueurs.
    pub fn determine_difficulty(&mut self) {
        let mut max_votes = 0;
        let mut selected_difficulty = Difficulty::Easy;

        for (&difficulty, &votes) in &self.difficulty_votes {
            if votes > max_votes { // Sélectionne la difficulté avec le plus de votes.
                max_votes = votes;
                selected_difficulty = difficulty;
            } else if votes == max_votes { // En cas d'égalité, sélection aléatoire entre les options en tête.
                if rand::thread_rng().gen_bool(0.5) {
                    selected_difficulty = difficulty;
                }
            }
        }

        self.set_difficulty(selected_difficulty); // Applique la difficulté déterminée.
    }

    // Démarre la phase de vote.
    pub fn start_voting_phase(&mut self) {
        self.phase = GamePhase::Voting; // Passe à la phase de vote.
        self.start_time = Some(Instant::now()); // Enregistre le moment de début de cette phase.
    }

    // Vérifie si la phase de vote est terminée (basée sur le countdown).
    pub fn check_voting_phase(&self) -> bool {
        if let Some(start_time) = self.start_time {
            return start_time.elapsed() >= Duration::new(20, 0); // Vérifie si 20 secondes se sont écoulées depuis le début de la phase de vote.
        }
        false
    }

    // Démarre la phase de jeu après le vote.
    pub fn start_game_phase(&mut self) {
        self.phase = GamePhase::Playing; // Passe à la phase de jeu.
        self.start_time = None; // Réinitialise le temps de début.
    }

    // Termine le jeu et réinitialise l'état.
    pub fn end_game(&mut self) {
        self.start_time = None; // Réinitialise le temps de début.
        self.players.clear(); // Vide la liste des joueurs.
        self.difficulty_votes.clear(); // Vide les votes de difficulté.
        self.phase = GamePhase::Identification; // Reviens à la phase d'identification.
    }
}


//Tests unitaires

#[cfg(test)]
mod tests {
    use super::*;

    // Test pour la création d'un nouveau jeu avec la difficulté par défaut
    #[test]
    fn test_game_new() {
        let game = Game::new();
        assert_eq!(game.difficulty, Difficulty::Easy);
        assert_eq!(game.players.len(), 0);
        assert_eq!(game.phase, GamePhase::Identification);
    }

    // Test pour ajouter un joueur
    #[test]
    fn test_add_player() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        assert_eq!(game.players.len(), 1);
        assert_eq!(*game.players.get("Player1").unwrap(), 0);
    }

    // Test pour enlever un joueur
    #[test]
    fn test_remove_player() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        game.remove_player("Player1");
        assert!(game.players.get("Player1").is_none());
    }

    // Test pour la devinette (plus)
    #[test]
    fn test_guess_more() {
        let mut game = Game::new();
        game.secret_number = 50;
        game.phase = GamePhase::Playing;
        let result = game.guess("Player1", 40).unwrap();
        assert_eq!(result, "C’est plus");
    }

    // Test pour la devinette (moins)
    #[test]
    fn test_guess_less() {
        let mut game = Game::new();
        game.secret_number = 50;
        game.phase = GamePhase::Playing;
        let result = game.guess("Player1", 60).unwrap();
        assert_eq!(result, "C’est moins");
    }

    // Test pour la devinette correcte
    #[test]
    fn test_guess_correct() {
        let mut game = Game::new();
        game.secret_number = 50;
        game.phase = GamePhase::Playing;
        let result = game.guess("Player1", 50).unwrap();
        assert_eq!(result, "Vous avez gagné!");
        assert_eq!(*game.players.get("Player1").unwrap(), 50);
    }

    // Test pour le vote de difficulté
    #[test]
    fn test_vote_difficulty() {
        let mut game = Game::new();
        game.vote_difficulty(Difficulty::Medium);
        assert_eq!(*game.difficulty_votes.get(&Difficulty::Medium).unwrap(), 1);
    }

    // Test pour déterminer la difficulté après le vote
    #[test]
    fn test_determine_difficulty() {
        let mut game = Game::new();
        game.vote_difficulty(Difficulty::Medium);
        game.vote_difficulty(Difficulty::Medium);
        game.vote_difficulty(Difficulty::Hard);
        game.determine_difficulty();
        assert_eq!(game.difficulty, Difficulty::Medium);
    }

    // Test pour démarrer la phase de vote
    #[test]
    fn test_start_voting_phase() {
        let mut game = Game::new();
        game.start_voting_phase();
        assert_eq!(game.phase, GamePhase::Voting);
        assert!(game.start_time.is_some());
    }

    // Test pour vérifier la durée de la phase de vote
    #[test]
    fn test_check_voting_phase() {
        let mut game = Game::new();
        game.start_voting_phase();
        std::thread::sleep(std::time::Duration::from_secs(21));
        assert!(game.check_voting_phase());
    }

    // Test pour démarrer la phase de jeu
    #[test]
    fn test_start_game_phase() {
        let mut game = Game::new();
        game.start_game_phase();
        assert_eq!(game.phase, GamePhase::Playing);
        assert!(game.start_time.is_none());
    }

    // Test pour terminer le jeu
    #[test]
    fn test_end_game() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        game.end_game();
        assert_eq!(game.players.len(), 0);
        assert_eq!(game.phase, GamePhase::Identification);
    }
}

