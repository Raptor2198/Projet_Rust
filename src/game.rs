use serde::{Deserialize, Serialize};

use crate::event::EventHandler;
use crate::map::{Map, MapTile};
use crate::player::Player;

#[derive(Serialize, Deserialize)]
pub enum GameEventInput {
    PlayerInput(usize, String),
}

// Container for every data in the game
// Same struct for every player
pub struct Game<M: MapTile, P, H> {
    pub handler: H,      // Handle for events, inputs / outputs, etc...
    pub players: Vec<P>, // Data for players, health, inventory, score, etc...
    pub map: Map<M>,     // Map data
}

// API to which you can interact with the game as a "user"
impl<M, P, H> Game<M, P, H>
where
    M: MapTile,
    P: Player,
    H: EventHandler,
{
    pub fn init(mapsize: (usize, usize)) -> Game<M, P, H> {
        Game {
            players: vec![],
            map: Map::init(mapsize.0, mapsize.1),
            handler: H::init(),
        }
    }

    pub fn spawn_player(&mut self, coord: (usize, usize)) -> usize {
        // Player id starts from 0
        let id = self.players.len();
        let mapid = self.map.add_player(coord);
        assert!(mapid == id, "Inequality id and mapid");
        self.players.push(P::init(id));
        id
    }

    pub fn handle_event(&mut self, evt: GameEventInput) -> Result<(), H::Error> {
        match evt {
            GameEventInput::PlayerInput(id, inp) => self.handler.handle(id, inp.as_str(), &mut self.map)?,
        }
        Ok(())
    }
}
