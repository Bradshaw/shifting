use crate::world::World;

mod world;

fn main() {
    let w = World::new(80, 20);
    for y in 0..20 {
        let mut s = String::new();
        for x in 0..80 {
            s.push(match w.get_tile(x, y) {
                None => '?',
                Some(tile) => tile.as_char(),
            });
        }
        println!("{s}");
    }
}
