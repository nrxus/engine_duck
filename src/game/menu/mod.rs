mod gui;

pub use self::gui::Quit;
use self::gui::Gui;
use asset;

use moho::{self, input};
use moho::font::Font;
use moho::errors::*;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

use std::rc::Rc;

#[derive(Default)]
pub struct Menu {
    gui: Gui,
}

impl Menu {
    pub fn update(self, input: &input::State) -> moho::State<Self, Quit> {
        self.gui.update(input).map(|gui| Menu { gui })
    }
}

pub struct Assets<T> {
    husky: Image<T>,
    duck: Image<T>,
    heart: Image<T>,
    instructions: Image<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(menu: &Menu, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let husky = {
            let pos = align::right(640 - 32 - 30).middle(125);
            asset_manager.image(asset::Texture::Husky, pos)?.scale(2)
        };
        let duck = {
            let pos = align::left(640 + 32 + 30).middle(125);
            asset_manager.image(asset::Texture::Duck, pos)?.scale(2)
        };
        let heart = {
            let pos = align::center(640).middle(125);
            asset_manager.image(asset::Texture::Heart, pos)?.scale(2)
        };
        let instructions = {
            let font = asset_manager.font(asset::Font::KenPixel, 32)?;
            let color = ColorRGBA(255, 255, 0, 255);
            let text = "<Use Arrow Keys to select option; then press Enter>";
            let texture = font.texturize(text, &color).map(Rc::new)?;
            let dims = texture.dims();
            let dst = align::bottom(720 - dims.y as i32).center(640).dims(dims);
            Image { texture, dst }
        };
        let gui = {
            let picker = asset_manager.texture(asset::Texture::Heart)?;
            let font = asset_manager.font(asset::Font::KenPixel, 64)?;
            gui::Assets::load(&*font, picker, &menu.gui)
        }?;

        Ok(Assets {
            husky,
            duck,
            heart,
            instructions,
            gui,
        })
    }
}

impl<T> Assets<T> {
    pub fn next(mut self, menu: &Menu) -> Self {
        self.gui = self.gui.next(&menu.gui);
        self
    }
}

impl<R: Renderer, T: Draw<R> + Texture> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.show(&self.heart)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}
