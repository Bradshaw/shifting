use std::collections::HashMap;
use crate::Structure;
use crate::utils::point::Point;
use rand::Rng;
use crate::world::tile::Tile;

pub mod structure;
mod tile;


pub struct World {
    structures: HashMap<Point, Structure>,
    empty: Tile
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut structures = HashMap::new();

        for _ in 0..10 {
            structures.insert(
                Point::new(rng.gen_range(0..80),rng.gen_range(0..50)),
                Structure::new(10,5)
            );
        }

        World {
            structures,
            empty: Tile::empty()
        }
    }
    pub fn get_tile(&self, x: usize, y:usize) -> &Tile {
        for (point, structure) in self.structures.iter() {
            if
                x >= point.x as usize &&
                x < (point.x as usize + structure.width) &&
                y >= point.y as usize &&
                y < (point.y as usize + structure.height) {
                match structure.get_tile(x-point.x as usize, y-point.y as usize) {
                    None => continue,
                    Some(tile) => return tile
                }
            }
        }
        &self.empty
    }
}