#[macro_use]
extern crate error_chain;
extern crate glm;
extern crate moho;
extern crate sdl2;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod data;
mod asset;
mod level_viewer;
mod game;

use moho::engine::{step, Engine};
use sdl2::image::{INIT_JPG, INIT_PNG};

pub type RefSnapshot<'a, W> = step::Snapshot<&'a W, &'a step::fixed::State>;

pub mod errors {
    error_chain!{
        links {
            Moho(::moho::errors::Error, ::moho::errors::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
        }
    }
}

fn main() {
    //Setup SDL
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let name = "Husky Loves Ducky";

    let sdl_ctx = sdl2::init().unwrap();
    let video_ctx = sdl_ctx.video().unwrap();
    let bounds = video_ctx.display_bounds(0).unwrap();
    let window = video_ctx
        .window(name, bounds.width(), bounds.height())
        .position_centered()
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    canvas
        .set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let _image_ctx = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let event_pump = sdl_ctx.event_pump().unwrap();
    let texture_loader = canvas.texture_creator();

    //Setup Moho
    let step = step::FixedUpdate::default().rate(30);
    let mut engine = Engine::new(event_pump, canvas, step);

    let level_viewer = std::env::args().any(|a| a == "--l");
    if level_viewer {
        level_viewer::run(&mut engine)
    } else {
        let font_loader = moho::renderer::sdl2::font::Loader::load(&texture_loader).unwrap();
        game::run(&mut engine, &texture_loader, &font_loader)
    }.unwrap()
}
