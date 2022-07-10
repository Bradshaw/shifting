use rand::seq::IteratorRandom;

pub struct World {
    map: Vec<Tile>,
    width: usize,
    //height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let mut map: Vec<Tile> = Vec::new();
        for _ in 0..(width * height) {
            map.push(Tile::new());
        }
        World {
            map,
            width,
            //height,
        }
    }
    pub fn get_tile(&self, _x: usize, _y: usize) -> Option<&Tile> {
        self.map.get(_x+_y*self.width)
    }
}

pub struct Tile {
    visrep: char
}

impl Tile {
    pub fn new() -> Tile {
        let mut rng = rand::thread_rng();
        let reps = "#. ∙";
        Tile {
            visrep: reps.chars().choose(&mut rng).unwrap()
        }
    }
    pub fn as_char(&self) -> char { self.visrep }
}