mod asset;
mod data;
mod game;
mod level_viewer;
mod utils;

use moho::engine::{step, Engine};
use sdl2::image::{INIT_JPG, INIT_PNG};

use std::rc::Rc;

type Result<T> = std::result::Result<T, failure::Error>;

struct Font<F>(F);

impl<F: moho::font::Font> moho::font::Font for Font<F> {
    type Texture = Rc<F::Texture>;

    fn measure(&self, text: &str) -> Result<glm::UVec2> {
        self.0.measure(text)
    }

    fn texturize(&self, text: &str, color: &moho::renderer::ColorRGBA) -> Result<Self::Texture> {
        self.0.texturize(text, color).map(Rc::new)
    }
}

struct FontLoader<FL>(FL);

impl<'t, T> FontLoader<moho::sdl2_helpers::font::Loader<'t, T>> {
    fn new(texture_loader: &'t sdl2::render::TextureCreator<T>) -> Self {
        FontLoader(moho::sdl2_helpers::font::Loader::load(texture_loader).unwrap())
    }
}

impl<'f, FL: moho::font::Loader<'f>> moho::font::Loader<'f> for FontLoader<FL> {
    type Font = Font<FL::Font>;
}

impl<'f, FL: moho::font::Loader<'f>> moho::resource::Loader<'f, Font<FL::Font>> for FontLoader<FL> {
    type Args = moho::font::Details;

    fn load(&'f self, data: &moho::font::Details) -> Result<Font<FL::Font>> {
        self.0.load(data).map(Font)
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
        let font_loader = FontLoader::new(&texture_loader);
        game::run(&mut engine, &texture_loader, &font_loader)
    }
    .unwrap()
}
