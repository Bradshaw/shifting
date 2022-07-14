use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use shifting::Shifting;
use shifting::constants::{COMPANY, GAME_ID, TITLE};

fn main() {
    let shifting = Shifting::new();
    
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new(GAME_ID, COMPANY)
        .window_setup(WindowSetup{
            title: TITLE.to_owned(),
            samples: NumSamples::One,
            vsync: false,
            icon: "".to_string(),
            srgb: false
        })
        .build()
        .expect("Could not create ggez context");
    
    graphics::set_mode(&mut ctx, WindowMode{
        width: 800.0*2.0,
        height: 600.0*2.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 800.0,
        min_height: 600.0,
        max_width: 800.0*2.0,
        max_height: 600.0*2.0,
        resizable: true,
        visible: true,
        resize_on_scale_factor_change: true
    }).expect("Could not set window mode");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}


// Shopping list:
// todo: entities
// todo: inventories?
// todo: world slices
// todo: generic renderer
// todo: vision
// todo: player interaction

