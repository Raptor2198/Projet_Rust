//ce fichier player.rs a été créé pour encapsuler la logique des joueurs, mais qu'il n'a pas encore été intégré ou utilisé dans les autres parties du code pour l'instant.

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

    // Test pour la création d'un joueur
    #[test]
    fn test_player_creation() {
        let player = Player::new("TestPlayer".to_string());
        // Vérifie que le nom du joueur est correct
        assert_eq!(player.name, "TestPlayer");
        // Vérifie que le score initial est 0
        assert_eq!(player.score, 0);
    }

    // Test pour incrémenter le score d'un joueur
    #[test]
    fn test_increment_score() {
        let mut player = Player::new("TestPlayer".to_string());
        player.increment_score();
        // Vérifie que le score est incrémenté de 1
        assert_eq!(player.score, 1);
    }
}
