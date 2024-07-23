use std::collections::HashMap;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::time::{Instant, Duration};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game {
    pub players: HashMap<String, u32>,
    secret_number: u32,
    pub difficulty: Difficulty,
    high_scores: Vec<(String, u32)>,
    difficulty_votes: HashMap<Difficulty, u32>,
    start_time: Option<Instant>,
    pub phase: GamePhase,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum GamePhase {
    Identification,
    Voting,
    Playing,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            secret_number: rand::thread_rng().gen_range(1..101),
            difficulty: Difficulty::Easy,
            high_scores: vec![],
            difficulty_votes: HashMap::new(),
            start_time: None,
            phase: GamePhase::Identification,
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.insert(name, 0);
    }

    pub fn remove_player(&mut self, name: &str) {
        self.players.remove(name);
    }
    
    pub fn guess(&mut self, player_name: &str, guess: u32) -> Result<&'static str, ()> {
        if self.phase != GamePhase::Playing {
            return Err(());
        }
        let hint = if guess < self.secret_number {
            "C’est plus"
        } else if guess > self.secret_number {
            "C’est moins"
        } else {
            self.players.insert(player_name.to_string(), guess);
            self.high_scores.push((player_name.to_string(), guess));
            "Vous avez gagné!"
        };
        Ok(hint)
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        match difficulty {
            Difficulty::Easy => self.secret_number = rand::thread_rng().gen_range(1..101),
            Difficulty::Medium => self.secret_number = rand::thread_rng().gen_range(1..501),
            Difficulty::Hard => self.secret_number = rand::thread_rng().gen_range(1..1001),
        }
    }

    pub fn vote_difficulty(&mut self, difficulty: Difficulty) {
        let count = self.difficulty_votes.entry(difficulty).or_insert(0);
        *count += 1;
    }

    pub fn determine_difficulty(&mut self) {
        let mut max_votes = 0;
        let mut selected_difficulty = Difficulty::Easy;

        for (&difficulty, &votes) in &self.difficulty_votes {
            if votes > max_votes {
                max_votes = votes;
                selected_difficulty = difficulty;
            } else if votes == max_votes {
                if rand::thread_rng().gen_bool(0.5) {
                    selected_difficulty = difficulty;
                }
            }
        }

        self.set_difficulty(selected_difficulty);
    }

    pub fn start_voting_phase(&mut self) {
        self.phase = GamePhase::Voting;
        self.start_time = Some(Instant::now());
    }

    pub fn check_voting_phase(&self) -> bool {
        if let Some(start_time) = self.start_time {
            return start_time.elapsed() >= Duration::new(20, 0);
        }
        false
    }

    pub fn start_game_phase(&mut self) {
        self.phase = GamePhase::Playing;
        self.start_time = None;
    }

    pub fn end_game(&mut self) {
        self.start_time = None;
        self.players.clear();
        self.difficulty_votes.clear();
        self.phase = GamePhase::Identification;
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

