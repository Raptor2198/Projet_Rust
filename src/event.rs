use crate::map::{Map, MapTile};

pub trait EventHandler {
    type Error;

    fn init() -> Self
    where
        Self: Sized;
    fn handle<M: MapTile>(
        &mut self,
        player: usize,
        inp: &str,
        map: &mut Map<M>,
    ) -> Result<(), Self::Error>;
}
