use std::io::Write;
use std::io::Stdout;

use crate::player::{PlayerDirection, PlayerMapData};

pub trait MapTile: Copy {
    fn init() -> Self
    where
        Self: Sized;
    fn write_stdout(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        self.repr(stdout)
    }
    fn repr<W: Write>(&self, buffer: &mut W) -> Result<(), std::io::Error>;
    fn set_player(&mut self, direction: PlayerDirection);
    fn walkable(&self) -> bool;
}

// Struct containing all the data required for the map to be displayed (and only this)
pub struct Map<T: MapTile> {
    players: Vec<PlayerMapData<T>>,

    pub width: usize,
    pub height: usize,
    map: Vec<Vec<T>>,
}

impl<T> Map<T>
where
    T: MapTile,
{
    pub fn init(width: usize, height: usize) -> Map<T> {
        let mut map = vec![];
        for _ in 0..height {
            let mut line = vec![];
            for _ in 0..width {
                line.push(T::init());
            }
            map.push(line);
        }

        Map {
            players: vec![],
            width,
            height,
            map,
        }
    }

    pub fn write_line_to_stdout(&self, y: usize, buffer: &mut Stdout) -> Result<(), std::io::Error> {
        // Get the line
        self.map
            .get(y)
            .expect("Called write_line with y too large")
            // Iterate over every tile on it
            .iter()
            // For each tile, call the `repr` function on the buffer
            .map(|t| t.write_stdout(buffer))
            // Collect the return types
            //    `Result<(), std::io::Error>` into a Vec<Result<(), std::io::Error>>`
            .collect()
        // A discrete conversion is being done here,
        //    it "squashes" a single `Result<(), std::io::Error>` from the Vec.
    }

    // Handy wrapper to get a ref to a tile from the coords
    // Because the data we reference is contained in `self`, it will "live" as long
    //    as `self` is alive, so the borrow checker is happy
    pub fn from_coord(&self, x: usize, y: usize) -> &T {
        self.map
            .get(y)
            .expect("Height > Map data lines")
            .get(x)
            .expect("Width > Map data columns")
    }

    // Handy wrapper to get a mutable ref to a tile from the coords
    pub fn mut_from_coord(&mut self, x: usize, y: usize) -> &mut T {
        self.map
            .get_mut(y)
            .expect("Height > Map data lines")
            .get_mut(x)
            .expect("Width > Map data columns")
    }

    // Handy wrapper to directly get a mut ref to the tile under a player
    pub fn get_player_tile(&mut self, id: usize) -> &mut T {
        let coord = self.players.get(id).expect("Id > player len").coord;
        self.mut_from_coord(coord.0, coord.1)
    }

    pub fn move_player(&mut self, x: isize, y: isize, player: usize) {
        assert!((x != 0) || (y != 0), "Empty move");
        let src = self.players.get(player).expect("Id > player len").coord;
        let direction = PlayerDirection::from_move(x, y).unwrap();

        // Computing destination tile
        let final_x = coord_move(src.0, x, 0, self.width - 1);
        let final_y = coord_move(src.1, y, 0, self.height - 1);
        let dst = (final_x, final_y);

        // No movement, set the player direction
        if src == dst {
            self.get_player_tile(player).set_player(direction);
            return;
        }

        // Get tile under destination, update player_data coords
        // We can't call `self.from_coord` while holding a `&mut` reference to
        //   `self.players`, so we have to call it before.
        // The `clone` will copy the data, thus destroying the reference
        let dst_tile = *self.from_coord(dst.0, dst.1);
        if !dst_tile.walkable() {
            self.get_player_tile(player).set_player(direction);
            return;
        }
        let player_data = self.players.get_mut(player).expect("Id > player len");
        player_data.coord = dst;

        // The player data registers what will become the tile under the player,
        //     and get the tile that was under the player before
        let src_tile = player_data.replace_tile_under(dst_tile);

        // Replace the player tile with the tile that was under
        *self.mut_from_coord(src.0, src.1) = src_tile;

        // Replace the destination tile with the player tile
        self.get_player_tile(player).set_player(direction);
    }

    // Add a new player to the map
    pub fn add_player(&mut self, coord: (usize, usize)) -> usize {
        let data = PlayerMapData::init(*self.from_coord(coord.0, coord.1), coord);
        self.mut_from_coord(data.coord.0, data.coord.1)
            .set_player(data.direction);
        self.players.push(data);
        self.players.len() - 1
    }
}

// Change the coord using a relative movement number
// Check the bounds, cap to min if too low, saturate to max if too high
fn coord_move(coord: usize, nmove: isize, min: usize, max: usize) -> usize {
    let fcoord = nmove.saturating_add_unsigned(coord);

    // Fancy way to do a `if fcoord < min`, only here we use usize / isize
    if fcoord.saturating_sub_unsigned(min).is_negative() {
        // If fcoord < min, we return min
        min
    } else {
        // Min is usize -> Always > 0
        // So we know fcoord > 0, which will not trigger error if converted to usize
        let fcoord: usize = fcoord.try_into().unwrap();

        // If fcoord > max, saturate to max
        fcoord.min(max)
    }
}

#[test]
fn test_coord_move() {
    assert_eq!(coord_move(0, -1, 0, 5), 0);
    assert_eq!(coord_move(1, -1, 0, 5), 0);
    assert_eq!(coord_move(3, -2, 0, 5), 1);
    assert_eq!(coord_move(4, 1, 0, 5), 5);
    assert_eq!(coord_move(4, 2, 0, 5), 5);
}
