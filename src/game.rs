use std::collections::HashMap;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game {
    players: HashMap<String, u32>,
    secret_number: u32,
    difficulty: Difficulty,
    high_scores: Vec<(String, u32)>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            secret_number: 0,  // sera initialisé par set_difficulty
            difficulty: Difficulty::Medium,
            high_scores: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.insert(name, 0);
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.secret_number = Self::generate_secret_number(difficulty);
    }

    pub fn generate_secret_number(difficulty: Difficulty) -> u32 {
        let mut rng = rand::thread_rng();
        match difficulty {
            Difficulty::Easy => rng.gen_range(1..=50),
            Difficulty::Medium => rng.gen_range(1..=100),
            Difficulty::Hard => rng.gen_range(1..=200),
        }
    }

    pub fn guess(&mut self, name: &str, guess: u32) -> Result<&str, &str> {
        match guess.cmp(&self.secret_number) {
            std::cmp::Ordering::Less => Ok("C'est plus!"),
            std::cmp::Ordering::Greater => Ok("C'est moins!"),
            std::cmp::Ordering::Equal => {
                if let Some(score) = self.players.get_mut(name) {
                    *score += 1;
                }
                self.secret_number = Self::generate_secret_number(self.difficulty);  // Reset the game
                Ok("Vous avez gagné!")
            }
        }
    }

    pub fn get_scores(&self) -> String {
        let mut scores = String::new();
        for (name, score) in &self.players {
            scores.push_str(&format!("{}: {}\n", name, score));
        }
        scores
    }

    pub fn get_high_scores(&self) -> String {
        let mut scores = String::new();
        for (i, (name, score)) in self.high_scores.iter().enumerate() {
            scores.push_str(&format!("{}. {}: {}\n", i + 1, name, score));
        }
        scores
    }

    pub fn update_high_scores(&mut self) {
        let mut high_scores: Vec<(String, u32)> = self.players.iter().map(|(name, &score)| (name.clone(), score)).collect();
        high_scores.sort_by(|a, b| b.1.cmp(&a.1)); // Sort in descending order
        self.high_scores = high_scores;
    }
}
