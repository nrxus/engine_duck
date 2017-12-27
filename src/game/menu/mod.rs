mod gui;

pub use self::gui::Quit;
use self::gui::Gui;
use asset;
use game;

use moho::input;
use moho::font::Font;
use moho::errors::*;
use moho::engine::{NextScene, World};
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

use std::rc::Rc;
use std::time::Duration;

#[derive(Default)]
pub struct Menu {
    gui: Gui,
}

impl World for Menu {
    type Quit = Quit;

    fn update(self, input: &input::State, elapsed: Duration) -> game::State<Self> {
        self.gui.update(input, elapsed).map(|gui| Menu { gui })
    }
}

impl<T: Texture> NextScene<Menu, (), ()> for Assets<T> {
    fn next(mut self, menu: &Menu, _: &(), _: &mut ()) -> Result<Self> {
        self.gui = self.gui.next(&menu.gui, &(), &mut ())?;
        Ok(self)
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
    pub fn load<AM>(asset_manager: &mut AM, menu: &Menu) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let husky = {
            let pos = align::right(640 - 32 - 30).middle(125);
            let mut image = asset_manager.image(asset::Texture::Husky, pos)?;
            image.dst = image.dst.scale(2);
            image
        };
        let duck = {
            let pos = align::left(640 + 32 + 30).middle(125);
            let mut image = asset_manager.image(asset::Texture::Duck, pos)?;
            image.dst = image.dst.scale(2);
            image
        };
        let heart = {
            let pos = align::center(640).middle(125);
            let mut image = asset_manager.image(asset::Texture::Heart, pos)?;
            image.dst = image.dst.scale(2);
            image
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

impl<R: Renderer, T: Draw<R> + Texture> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.show(&self.heart)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}
