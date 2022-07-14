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
        for _ in 0..width {
            for _ in 0..height {
                map.push(Tile::Wall);
            }
        }
        Structure {
            map,
            width,
            height
        }
    }
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        let index = x + y * self.width;
        if x>=self.width || y>=self.height || index>=self.map.len() {
            None
        } else {
            self.map.get(index)
        }
    }
}
