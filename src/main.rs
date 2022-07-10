use crate::world::World;
use std::io::{stdout};
use crossterm::{execute};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor};

mod world;

fn main() {
    let width = 80;
    let height = 20;
    let w = World::new(width, height);
    for y in 0..height {
        for x in 0..width {
            match w.get_tile(x, y) {
                None => execute!(
                    stdout(),
                    Print("?".to_string())
                ),
                Some(tile) => execute!(
                    stdout(),
                    SetForegroundColor(tile.foreground),
                    SetBackgroundColor(tile.background),
                    Print(tile.as_char().to_string()),
                    ResetColor
                ),
            }.expect("Could not print tile");
        }
        execute!(stdout(), Print("\n".to_string())).expect("Could not add a newline");
    }
}
