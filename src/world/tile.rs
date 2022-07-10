use rand::seq::IteratorRandom;
use crossterm::style::Color;

pub struct Tile {
    visual: char,
    pub foreground: Color,
    pub background: Color,
}

impl Tile {
    pub fn empty() -> Tile {
        Tile {
            visual: '⍽',
            background: Color::Reset,
            foreground: Color::DarkGreen,
        }
    }
    pub fn new() -> Tile {
        let mut rng = rand::thread_rng();
        let reps = "#. ∙";
        let backgrounds = [
            Color::Black, Color::DarkGrey,
            Color::DarkRed, Color::DarkGreen, Color::DarkBlue,
            Color::DarkCyan, Color::DarkMagenta, Color::DarkYellow
        ];
        let foregrounds = [
            Color::White, Color::Grey,
            Color::Red, Color::Green, Color::Blue,
            Color::Cyan, Color::Magenta, Color::Yellow
        ];

        Tile {
            visual: reps.chars().choose(&mut rng).unwrap(),
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
            background: *backgrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn as_char(&self) -> char { self.visual }
}