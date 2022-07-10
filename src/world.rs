use rand::seq::IteratorRandom;
use crossterm::style::Color;

pub struct World {
    map: Vec<Tile>,
    width: usize,
    height: usize
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

pub struct Tile {
    visrep: char,
    pub foreground: Color,
    pub background: Color,
}

impl Tile {
    pub fn new() -> Tile {
        let mut rng = rand::thread_rng();
        let reps = "#. ∙";
        let backgrounds = [
            Color::Black, Color::DarkGrey,
            Color::DarkRed,Color::DarkGreen,Color::DarkBlue,
            Color::DarkCyan,Color::DarkMagenta,Color::DarkYellow
        ];
        let foregrounds = [
            Color::White, Color::Grey,
            Color::Red,Color::Green,Color::Blue,
            Color::Cyan,Color::Magenta,Color::Yellow
        ];

        Tile {
            visrep: reps.chars().choose(&mut rng).unwrap(),
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
            background: *backgrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn as_char(&self) -> char { self.visrep }
}