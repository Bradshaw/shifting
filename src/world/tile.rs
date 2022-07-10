use std::io::Stdout;
use std::ops::BitAnd;
use crossterm::{QueueableCommand, Result};
use rand::seq::IteratorRandom;
use crossterm::style::{Attributes, Color, Print, ResetColor, SetAttribute, SetAttributes, SetBackgroundColor, SetForegroundColor};
use crossterm::style::Attribute::{Bold, Framed, Reset};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Floor(TileStyle),
    Wall(TileStyle),
    Water,
    Bug,
    Transparent,
    Unknown,
}

impl Tile {
    
    
    pub fn empty() -> Tile {
        match rand::thread_rng().gen_range(0..100){
            0 => Tile::Floor(TileStyle::grass()),
            1 => Tile::Floor(TileStyle::grass()),
            2 => Tile::Floor(TileStyle::grass()),
            3 => Tile::Floor(TileStyle::grass()),
            4 => Tile::Floor(TileStyle::grass()),
            5 => Tile::Floor(TileStyle::rock()),
            6 => Tile::Water,
            7 => Tile::Water,
            8 => Tile::Bug,
            _ => Tile::Floor(TileStyle::empty())
        }
    }
    
    
    pub fn wall() -> Tile {
        Tile::Wall(TileStyle::wall())
    }
    pub fn draw(&self, mut stdout: Stdout) -> Result<Stdout> {
        return match self {
            Tile::Floor(style) => {
                Tile::draw_drawable_tile(*style, stdout)
            }
            Tile::Wall(style) => {
                stdout.queue(SetAttributes(Attributes::from(Bold).bitand(Framed)))?;
                stdout = Tile::draw_drawable_tile(*style, stdout)?;
                stdout.queue(SetAttribute(Reset))?;
                Ok(stdout)
            }
            Tile::Water => {
                Tile::draw_drawable_tile(TileStyle::water(), stdout)
            }
            Tile::Bug => {
                Tile::draw_drawable_tile(TileStyle::bug(), stdout)
            }
            Tile::Transparent => {
                Tile::draw_arbitrary_tile(' ', stdout)
            }
            Tile::Unknown => {
                Tile::draw_arbitrary_tile('?', stdout)
            }
        }
    }
    fn draw_drawable_tile(style: TileStyle, mut stdout: Stdout) -> Result<Stdout>{
        stdout
            .queue(SetForegroundColor(style.foreground))?
            .queue(SetBackgroundColor(style.background))?
            .queue(Print(style.as_char().to_string()))?
            .queue(ResetColor)?;
        Ok(stdout)
    }
    fn draw_arbitrary_tile(c: char, mut stdout: Stdout) -> Result<Stdout>{
        stdout.queue(Print(c.to_string()))?;
        Ok(stdout)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TileStyle {
    visual: char,
    pub foreground: Color,
    pub background: Color,
}

impl TileStyle {
    pub fn empty() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "                                                      ⋅";
        let foregrounds = [
            Color::DarkGrey,
        ];
        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            background: Color::Reset,
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn grass() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "ᴗ₋,";
        let foregrounds = [
            Color::DarkGreen,
            Color::DarkYellow,
        ];
        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            background: Color::Reset,
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    
    
    pub fn water() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "˜∼∽≋";
        let foregrounds = [
            Color::White,
            Color::Cyan,Color::Cyan,Color::Cyan,Color::Cyan,
            Color::Blue,Color::Blue,Color::Blue,Color::Blue,
        ];
        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            background: Color::Reset,
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    
    
    
    
    pub fn bug() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "‚․‿⁔⁖∴∵∙⋅";
        let foregrounds = [
            Color::Black
        ];
        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            background: Color::Reset,
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn rock() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "o∙ѻ●⚬";
        let foregrounds = [
            Color::DarkGrey,
        ];
        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            background: Color::Reset,
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn wall() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "#█▓";
        let backgrounds = [
            Color::Black, Color::DarkGrey
        ];
        let foregrounds = [
            Color::DarkGrey, Color::Grey
        ];

        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
            background: *backgrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn color_wall() -> TileStyle {
        let mut rng = rand::thread_rng();
        let reps = "#█▓";
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

        TileStyle {
            visual: reps.chars().choose(&mut rng).unwrap(),
            foreground: *foregrounds.iter().choose(&mut rng).unwrap(),
            background: *backgrounds.iter().choose(&mut rng).unwrap(),
        }
    }
    pub fn as_char(&self) -> char { self.visual }
}