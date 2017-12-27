mod gui;
mod guide;

pub use self::gui::ButtonKind as PlayerKind;
use self::gui::Gui;
use self::guide::Guide;
use data::Animators;
use game;
use asset;

use moho::input;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

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
}

impl World for PlayerSelect {
    type Quit = PlayerKind;

    fn update(self, input: &input::State, elapsed: Duration) -> game::State<Self> {
        let guide = self.guide;

        self.gui.update(input, elapsed).map(|gui| {
            let guide = guide.update(input, elapsed).get();
            PlayerSelect { gui, guide }
        })
    }
}

pub struct Assets<T> {
    title: Image<T>,
    instructions: Image<T>,
    guide: guide::Assets<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> NextScene<PlayerSelect, (), ()> for Assets<T> {
    fn next(mut self, world: &PlayerSelect, _: &(), _: &mut ()) -> Result<Self> {
        self.guide = self.guide.next(&world.guide, &(), &mut ())?;
        self.gui = self.gui.next(&world.gui, &(), &mut ())?;
        Ok(self)
    }
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        let font = asset_manager.font(asset::Font::KenPixel, 64)?;
        let title = font.texturize("Select Player", &color)?
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
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.guide)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}
