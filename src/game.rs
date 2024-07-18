use std::collections::HashMap;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize,Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game {
    pub players: HashMap<String, u32>,
    secret_number: u32,
    difficulty: Difficulty,
    high_scores: Vec<(String, u32)>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            secret_number: rand::thread_rng().gen_range(1..101),
            difficulty: Difficulty::Easy,
            high_scores: vec![],
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.insert(name, 0);
    }

    pub fn guess(&mut self, player_name: &str, guess: u32) -> Result<&'static str, ()> {
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
    }
}
