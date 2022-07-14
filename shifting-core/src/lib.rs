use crate::world::World;

pub mod constants;
pub mod utils;
pub mod world;

pub struct Shifting {
    pub title: String,
    pub world: World,
}

impl Shifting {
    pub fn new() -> Shifting {
        Shifting {
            title: "Shifting".to_string(),
            world: World::new(),
        }
    }
}
