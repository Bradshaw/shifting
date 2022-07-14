mod utils;
mod world;
pub mod constants;
mod renderer;


pub struct Shifting {
    pub title: String
}

impl Shifting {
    pub fn new() -> Shifting {
        Shifting{title: "Shifting".to_string()}
    }
}
