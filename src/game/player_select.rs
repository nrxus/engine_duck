use game::font;

use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

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
        let title = font.texturize("Select Player", &color)?
            .at(align::top(50).center(640));
        let collect = font.texturize("Collect", &color)?
            .at(align::top(400).center(960));
        let avoid = font.texturize("Avoid", &color)?
            .at(align::top(400).center(320));

        let instructions = {
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let text = "<Use Arrow Keys to choose player; then press Enter>";
            let height = font.measure(text)?.y as i32;
            font.texturize(text, &color)?
                .at(align::bottom(720 - height).center(640))
        };

        Ok(Assets {
            title,
            collect,
            avoid,
            instructions,
        })
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.collect)?;
        renderer.show(&self.avoid)?;
        renderer.show(&self.instructions)
    }
}
