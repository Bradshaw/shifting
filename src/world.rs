use std::collections::HashMap;
use crate::Structure;
use crate::utils::point::Point;
use rand::Rng;
use crate::world::tile::Tile;

pub mod structure;
mod tile;


pub struct World {
    structures: HashMap<Point, Structure>,
    pregenerated: HashMap<(usize, usize), Tile>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut structures = HashMap::new();

        for _ in 0..10 {
            structures.insert(
                Point::new(rng.gen_range(0..80),rng.gen_range(0..40)),
                Structure::new(10,5)
            );
        }

        World {
            structures,
            pregenerated: HashMap::new()
        }
    }
    pub fn get_tile(&mut self, x: usize, y:usize) -> Tile {
        match self.pregenerated.get(&(x, y)){
            None => {
                let tile = self.get_actual(x, y);
                self.pregenerated.insert((x, y), tile);
                return tile;
            }
            Some(tile) => *tile
        }
    }
    pub fn get_actual(&self, x: usize, y: usize) -> Tile {
        for (point, structure) in self.structures.iter() {
            if
            x >= point.x as usize &&
                x < (point.x as usize + structure.width) &&
                y >= point.y as usize &&
                y < (point.y as usize + structure.height) {
                match structure.get_tile(x-point.x as usize, y-point.y as usize) {
                    None => continue,
                    Some(tile) => return *tile
                }
            }
        }
        return Tile::empty();
    }
}