use rand::Rng;

#[derive(Copy, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Water,
    Bug,
    Transparent,
    Unknown,
}

impl Tile {
    pub fn empty() -> Tile {
        match rand::thread_rng().gen_range(0..100){
            0 => Tile::Floor,
            1 => Tile::Floor,
            2 => Tile::Floor,
            3 => Tile::Floor,
            4 => Tile::Floor,
            5 => Tile::Floor,
            6 => Tile::Water,
            7 => Tile::Water,
            8 => Tile::Bug,
            _ => Tile::Floor
        }
    }
}