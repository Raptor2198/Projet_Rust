use crate::map::MapTile;

#[derive(Clone, Copy)]
pub enum PlayerDirection {
    North,
    South,
    East,
    West,
}

impl PlayerDirection {
    // Decide of a display direction depending on a movement
    pub fn from_move(x: isize, y: isize) -> Option<PlayerDirection> {
        if x < 0 {
            Some(PlayerDirection::West)
        } else if y < 0 {
            Some(PlayerDirection::North)
        } else if x > 0 {
            Some(PlayerDirection::East)
        } else if y > 0 {
            Some(PlayerDirection::South)
        } else {
            None
        }
    }
}

// Struct containing map-related metadata about a player
pub struct PlayerMapData<T: MapTile> {
    pub coord: (usize, usize),
    tile_under: T,
    pub direction: PlayerDirection,
}

impl<T: MapTile> PlayerMapData<T> {
    pub fn init(tile_under: T, coord: (usize, usize)) -> PlayerMapData<T> {
        PlayerMapData {
            coord,
            tile_under,
            direction: PlayerDirection::North, // Some default direction
        }
    }

    pub fn replace_tile_under(&mut self, new_tile: T) -> T {
        std::mem::replace(&mut self.tile_under, new_tile)
    }
}

// Trait containing every behavior expected from a Player
pub trait Player {
    fn init(id: usize) -> Self
    where
        Self: Sized;
}
