use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, EventLoop};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use shifting_core::constants::*;
use shifting_core::world::tile::Tile;
use shifting_core::Shifting;

pub struct Renderer {
    game: Shifting,
}

impl Renderer {
    pub fn run(mut renderer: Renderer) {
        let title = renderer.game.title.as_str();
        let (mut ctx, event_loop) = ContextBuilder::new(GAME_ID, COMPANY)
            .window_setup(WindowSetup {
                title: format!("{title}"),
                samples: NumSamples::One,
                vsync: false,
                icon: "".to_string(),
                srgb: false,
            })
            .build()
            .expect("Could not create ggez context");

        graphics::set_mode(
            &mut ctx,
            WindowMode {
                width: 800.0 * 2.0,
                height: 600.0 * 2.0,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 800.0,
                min_height: 600.0,
                max_width: 800.0 * 2.0,
                max_height: 600.0 * 2.0,
                resizable: true,
                visible: true,
                resize_on_scale_factor_change: true,
            },
        )
        .expect("Could not set window mode");

        event::run(ctx, event_loop, renderer);
    }

    pub fn new() -> Renderer {
        Renderer {
            game: Shifting::new(),
        }
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        graphics::present(ctx)
    }
}
