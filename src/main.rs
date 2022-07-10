mod utils;
mod world;

use world::structure::Structure;
use std::io::stdout;
use crossterm::execute;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crate::world::World;

fn main() {
    let width = 90;
    let height = 45;
    let w = World::new();
    for y in 0..height {
        for x in 0..width {
            let tile = w.get_tile(x, y);
            execute!(
                stdout(),
                SetForegroundColor(tile.foreground),
                SetBackgroundColor(tile.background),
                Print(tile.as_char().to_string()),
                ResetColor
            ).expect("Could not display tile");
        }
        execute!(stdout(), Print("\n".to_string())).expect("Could not add a newline");
    }
}
