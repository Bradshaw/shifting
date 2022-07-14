extern crate shifting_renderer_ggez;

use shifting_renderer_ggez::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    Renderer::run(renderer);
}

// Shopping list:
// todo: entities
// todo: inventories?
// todo: world slices
// todo: generic renderer
// todo: vision
// todo: player interaction
