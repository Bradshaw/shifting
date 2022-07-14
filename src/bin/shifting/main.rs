extern crate shifting_renderer_ggez;

use shifting_renderer_ggez::Renderer;

fn main() {
    let renderer = Renderer::new();
    match Renderer::run(renderer) {
        Ok(_) => println!("Thank you for playing"),
        Err(e) => println!("Something donked up: {e}"),
    };
}

// Shopping list:
// todo: entities
// todo: inventories?
// todo: world slices
// todo: generic renderer
// todo: vision
// todo: player interaction
