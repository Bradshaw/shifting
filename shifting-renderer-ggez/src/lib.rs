use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, FilterMode, Image, Rect, Transform};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use mint::*;
use shifting_core::constants::*;
use shifting_core::world::tile::Tile;
use shifting_core::Shifting;
use std::fs::File;
use std::io::Read;

pub struct Renderer {
    game: Shifting,
    tileset: Option<Image>,
}

impl Renderer {
    pub fn run(mut renderer: Renderer) -> Result<(), GameError> {
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

        let mut f = File::open("./resources/monochrome_tilemap_packed.png")?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;

        let mut image = Image::from_bytes(&mut ctx, buffer.as_slice())?;
        image.set_filter(FilterMode::Nearest);

        renderer.tileset = Some(image);

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

        event::run(ctx, event_loop, renderer)
    }

    pub fn new() -> Renderer {
        Renderer {
            game: Shifting::new(),
            tileset: None,
        }
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let tileset = match &self.tileset {
            None => panic!("Could not find tileset"),
            Some(t) => t,
        };

        for x in 0..40 {
            for y in 0..30 {
                match self.game.world.get_tile(x, y) {
                    Tile::Floor => sprite_at(
                        ctx,
                        tileset,
                        5,
                        4,
                        x as i32,
                        y as i32,
                        Color::from_rgb(127, 64, 12),
                    )?,
                    Tile::Wall => sprite_at(
                        ctx,
                        tileset,
                        12,
                        6,
                        x as i32,
                        y as i32,
                        Color::from_rgb(127, 127, 180),
                    )?,
                    Tile::Water => sprite_at(
                        ctx,
                        tileset,
                        4,
                        4,
                        x as i32,
                        y as i32,
                        Color::from_rgb(127, 200, 200),
                    )?,
                    Tile::Bug => sprite_at(
                        ctx,
                        tileset,
                        6,
                        5,
                        x as i32,
                        y as i32,
                        Color::from_rgb(64, 127, 40),
                    )?,
                    Tile::Transparent => {}
                    Tile::Unknown => {}
                }
            }
        }
        graphics::present(ctx)
    }
}

fn circle_at(ctx: &mut Context, x: i32, y: i32, col: Color) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Point2 { x: 0.0, y: 0.0 },
        10.0,
        2.0,
        Color::WHITE,
    )?;
    graphics::draw(
        ctx,
        &circle,
        DrawParam {
            src: Rect {
                x: 0.0,
                y: 0.0,
                w: 16.0,
                h: 16.0,
            },
            color: col,
            trans: Transform::Values {
                dest: Point2 {
                    x: 0.0 + (x * 16) as f32,
                    y: 0.0 + (y * 16) as f32,
                },
                rotation: 0.0,
                scale: Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
        },
    )
}

fn sprite_at(
    ctx: &mut Context,
    image: &Image,
    sx: i32,
    sy: i32,
    x: i32,
    y: i32,
    col: Color,
) -> GameResult {
    let cx: i32 = (image.width() / 8) as i32;
    let cy: i32 = (image.height() / 8) as i32;

    let w: f32 = 1.0 / cx as f32;
    let h: f32 = 1.0 / cy as f32;

    graphics::draw(
        ctx,
        image,
        DrawParam {
            src: Rect {
                x: sx as f32 * w,
                y: sy as f32 * h,
                w: w as f32,
                h: h as f32,
            },
            color: col,
            trans: Transform::Values {
                dest: Point2 {
                    x: 0.0 + (x * 32) as f32,
                    y: 0.0 + (y * 32) as f32,
                },
                rotation: 0.0,
                scale: Vector2 { x: 4.0, y: 4.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            },
        },
    )
}
