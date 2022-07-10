mod utils;
mod world;

use world::structure::Structure;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{ExecutableCommand, QueueableCommand, Result, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Print};
use crate::world::World;

fn main() -> Result<()> {
    let width = 90;
    let height = 45;
    let mut w = World::new();
    stdout().execute(crossterm::cursor::Hide)?;
    //for frame in 0..100 {
    stdout().queue(terminal::Clear(terminal::ClearType::All))?;
    loop {
        let mut stdout = stdout();
        for y in 0..height {
            for x in 0..width {
                stdout.queue(MoveTo(x, y))?;
                let tile = w.get_tile(x.into(), y.into());
                stdout = tile.draw(stdout)?;
            }
        }
        sleep(Duration::new(0,10_000_000));
    }
    Ok(())
}

