#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

impl Player {
    // Crée un nouveau joueur avec un nom et un score initial de 0
    pub fn new(name: String) -> Player {
        Player { name, score: 0 }
    }

    // Incrémente le score du joueur
    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}

// Tests unitaires pour le module player
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("TestPlayer".to_string());
        assert_eq!(player.name, "TestPlayer");
        assert_eq!(player.score, 0);
    }

    #[test]
    fn test_increment_score() {
        let mut player = Player::new("TestPlayer".to_string());
        player.increment_score();
        assert_eq!(player.score, 1);
    }
}
