#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { name, score: 0 }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}
