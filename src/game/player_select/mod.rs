mod gui;
mod guide;

pub use self::gui::ButtonKind as PlayerKind;
use self::gui::Gui;
use self::guide::Guide;
use data::Animators;
use {asset, Result};

use moho::font::Font;
use moho::renderer::{align, ColorRGBA};
use moho::texture::{Image, Texture};
use moho::{self, input};

use std::time::Duration;

pub struct PlayerSelect {
    guide: Guide,
    gui: Gui,
}

impl PlayerSelect {
    pub fn new(animators: &Animators) -> Self {
        PlayerSelect {
            guide: Guide::new(animators),
            gui: Gui::new(animators),
        }
    }

    pub fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, PlayerKind> {
        let guide = self.guide;

        self.gui.update(input, elapsed).map(|gui| {
            let guide = guide.update(elapsed);
            PlayerSelect { gui, guide }
        })
    }
}

#[derive(Show)]
pub struct Assets<T> {
    title: Image<T>,
    instructions: Image<T>,
    guide: guide::Assets<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture + Clone> Assets<T> {
    pub fn load(asset_manager: &mut impl asset::Manager<Texture = T>) -> Result<Self> {
        let color = ColorRGBA(255, 255, 0, 255);

        let font = asset_manager.font(asset::Font::KenPixel, 64)?;
        let title = font
            .texturize("Select Player", &color)?
            .at(align::top(50).center(640));

        let guide = guide::Assets::load(&*font, asset_manager)?;

        let instructions = {
            let font = asset_manager.font(asset::Font::KenPixel, 32)?;
            let text = "<Use Arrow Keys to choose player; then press Enter>";
            let height = font.measure(text)?.y as i32;
            font.texturize(text, &color)?
                .at(align::bottom(720 - height).center(640))
        };

        let gui = gui::Assets::load(asset_manager)?;

        Ok(Assets {
            title,
            guide,
            instructions,
            gui,
        })
    }

    pub fn next(mut self, world: &PlayerSelect) -> Self {
        self.guide = self.guide.next(&world.guide);
        self.gui = self.gui.next(&world.gui);
        self
    }
}
