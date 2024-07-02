#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Game, Difficulty};

    #[test]
    fn test_add_player() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        assert!(game.players.contains_key("Player1"));
    }

    #[test]
    fn test_guess() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        let secret_number = game.secret_number;
        let result = game.guess("Player1", secret_number).unwrap();
        assert_eq!(result, "Vous avez gagné!");
    }

    #[test]
    fn test_get_scores() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        game.players.insert("Player1".to_string(), 5);
        let scores = game.get_scores();
        assert!(scores.contains("Player1: 5"));
    }

    #[test]
    fn test_difficulty_easy() {
        let mut game = Game::new();
        game.set_difficulty(Difficulty::Easy);
        assert!(game.secret_number <= 50);
    }

    #[test]
    fn test_difficulty_medium() {
        let mut game = Game::new();
        game.set_difficulty(Difficulty::Medium);
        assert!(game.secret_number <= 100);
    }

    #[test]
    fn test_difficulty_hard() {
        let mut game = Game::new();
        game.set_difficulty(Difficulty::Hard);
        assert!(game.secret_number <= 200);
    }

    #[test]
    fn test_high_scores() {
        let mut game = Game::new();
        game.add_player("Player1".to_string());
        game.add_player("Player2".to_string());
        game.guess("Player1", game.secret_number).unwrap();
        game.guess("Player2", game.secret_number).unwrap();
        let high_scores = game.get_high_scores();
        assert!(high_scores.contains("Player1:"));
        assert!(high_scores.contains("Player2:"));
    }
}
