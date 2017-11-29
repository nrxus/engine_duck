use asset::{self, Image, Sprite};
use game::font::{self, FontExt};

use moho::errors::*;
use moho::renderer::{align, ColorRGBA, Font, Renderer, Scene, Texture};

pub struct PlayerSelect {}

pub struct Assets<T> {
    title: Image<T>,
    collect: Image<T>,
    avoid: Image<T>,
    instructions: Image<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<FM>(font_manager: &mut FM) -> Result<Self>
    where
        FM: font::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        let font = font_manager.load(font::Kind::KenPixel, 64)?;
        let title = font.image("Select Player", &color, align::top(50).center(640))?;
        let collect = font.image("Collect", &color, align::top(400).center(960))?;
        let avoid = font.image("Avoid", &color, align::top(400).center(320))?;

        let instructions = font_manager.load(font::Kind::KenPixel, 32).and_then(|f| {
            let text = "<Use Arrow Keys to choose player; then press Enter>";
            let height = f.measure(text)?.y as i32;
            f.image(text, &color, align::bottom(720 - height).center(640))
        })?;
        Ok(Assets {
            title,
            collect,
            avoid,
            instructions,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Assets<R::Texture> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.collect)?;
        renderer.show(&self.avoid)?;
        renderer.show(&self.instructions)
    }
}
