//todo: generate nice-looking structures

use crate::world::tile::Tile;

/// Represents some kind of limited-size structure, like a building or a village
pub struct Structure {
    map: Vec<Tile>,
    pub width: usize,
    pub height: usize
}

impl Structure {
    pub fn new(width: usize, height: usize) -> Structure {
        let mut map: Vec<Tile> = Vec::new();
        for _ in 0..(width * height) {
            map.push(Tile::wall());
        }
        Structure {
            map,
            width,
            height
        }
    }
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x>=self.width || y>=self.height{
            None
        } else {
            self.map.get(x + y *self.width)
        }
    }
}
