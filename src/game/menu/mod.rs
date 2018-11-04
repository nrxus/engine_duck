mod gui;

pub use self::gui::Quit;

use self::gui::Gui;
use crate::{asset, Result};

use moho::{
    self,
    font::Font,
    input,
    renderer::{align, ColorRGBA},
    texture::{Image, Texture},
};

#[derive(Default)]
pub struct Menu {
    gui: Gui,
}

impl Menu {
    pub fn update(self, input: &input::State) -> moho::State<Self, Quit> {
        self.gui.update(input).map(|gui| Menu { gui })
    }
}

#[derive(moho::Show)]
pub struct Assets<T> {
    husky: Image<T>,
    duck: Image<T>,
    heart: Image<T>,
    instructions: Image<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load(menu: &Menu, asset_manager: &mut impl asset::Manager<Texture = T>) -> Result<Self> {
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
            let texture = font.texturize(text, &color)?;
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
