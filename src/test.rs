use std::io::Write;
use std::time::Duration;

use crossterm::event::{Event, KeyCode};

use crate::event::EventHandler;
use crate::game::{Game, GameEventInput};
use crate::map::{Map, MapTile};
use crate::player::{Player, PlayerDirection};
use crate::tui::Terminal;

#[derive(Clone, Copy)]
pub enum MapTileTest {
    Grass,
    Wall,
    Player(PlayerDirection),
}

impl MapTile for MapTileTest {
    fn init() -> Self
    where
        Self: Sized,
    {
        MapTileTest::Grass
    }

    fn repr<W: Write>(&self, buffer: &mut W) -> Result<(), std::io::Error> {
        write!(
            buffer,
            "{}",
            match self {
                MapTileTest::Grass => "░",
                MapTileTest::Wall => "█",

                MapTileTest::Player(direction) => match direction {
                    PlayerDirection::North => "^",
                    PlayerDirection::South => "v",
                    PlayerDirection::East => ">",
                    PlayerDirection::West => "<",
                },
            }
        )
    }

    fn set_player(&mut self, direction: PlayerDirection) {
        *self = MapTileTest::Player(direction);
    }

    fn walkable(&self) -> bool {
        match self {
            MapTileTest::Grass => true,
            _ => false,
        }
    }
}

pub struct PlayerTest {}

impl Player for PlayerTest {
    fn init(_id: usize) -> Self
    where
        Self: Sized,
    {
        PlayerTest {}
    }
}

pub struct HandlerTest {}

impl EventHandler for HandlerTest {
    type Error = u32;

    fn init() -> Self
    where
        Self: Sized,
    {
        HandlerTest {}
    }

    fn handle<M: MapTile>(
        &mut self,
        player: usize,
        inp: &str,
        map: &mut Map<M>,
    ) -> Result<(), Self::Error> {
        match inp {
            "up" => map.move_player(0, -1, player),
            "down" => map.move_player(0, 1, player),
            "right" => map.move_player(1, 0, player),
            "left" => map.move_player(-1, 0, player),
            _ => return Err(1),
        }
        Ok(())
    }
}

type GameTest = Game<MapTileTest, PlayerTest, HandlerTest>;

fn square<T: MapTile>(map: &mut Map<T>, tile: T, start: (usize, usize), size: (usize, usize)) {
    for x in start.0..(start.0 + size.0) {
        let x = x.min(map.width - 1);
        for y in start.1..(start.1 + size.1) {
            let y = y.min(map.height - 1);
            *map.mut_from_coord(x, y) = tile;
        }
    }
}

fn prepare_map(game: &mut GameTest) {
    square(&mut game.map, MapTileTest::Wall, (5, 3), (10, 4));
}

pub fn main() {
    let mut term = Terminal::init().expect("Error while setting the terminal");
    let mut game: GameTest = Game::init((50, 8));
    let id_1 = game.spawn_player((0, 0));

    prepare_map(&mut game);

    let time_refresh = Duration::from_millis(500);
    loop {
        term.display_map(&game.map)
            .expect("Error while displaying the map");

        if let Some(evt) = term
            .poll_event(Some(time_refresh))
            .expect("Error while getting terminal event")
        {
            if let Event::Key(k) = evt {
                let evt = match k.code {
                    KeyCode::Left => Some(GameEventInput::PlayerInput(id_1, "left".to_string())),
                    KeyCode::Right => Some(GameEventInput::PlayerInput(id_1, "right".to_string())),
                    KeyCode::Up => Some(GameEventInput::PlayerInput(id_1, "up".to_string())),
                    KeyCode::Down => Some(GameEventInput::PlayerInput(id_1, "down".to_string())),
                    KeyCode::Char('q') => break,
                    _ => None,
                };

                if let Some(evt) = evt {
                    game.handle_event(evt)
                        .expect("Error while applying game event input");
                }
            }
        }
    }
}
