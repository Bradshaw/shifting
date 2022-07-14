use std::collections::HashMap;
use crate::utils::point::Point;
use rand::Rng;
use crate::world::structure::Structure;
use crate::world::tile::Tile;

pub mod structure;
mod tile;


pub struct World {
    structures: HashMap<Point, Structure>,
    //todo: are hashmaps keyed with (usize, usize) efficient
    //todo: use Point instead of (usize, usize)
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
        //todo: use quadtrees or something to avoid iterating over all structures
        for (point, structure) in self.structures.iter() {
            if
            x >= point.x as usize &&
                x < (point.x as usize + structure.width) &&
                y >= point.y as usize &&
                y < (point.y as usize + structure.height) {
                match structure.get_tile(x-point.x as usize, y-point.y as usize) {
                    None => continue,
                    Some(tile) => {
                        match tile {
                            Tile::Transparent => continue,
                            Tile::Unknown => continue,
                            _ => return *tile,
                        }
                    }
                }
            }
        }
        return Tile::empty();
    }
}