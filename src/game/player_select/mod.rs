mod gui;
mod collect;
mod avoid;

pub use self::gui::ButtonKind as PlayerKind;
use self::gui::Gui;
use self::collect::Collect;
use self::avoid::Avoid;
use data::{self, Animators};
use game::{self, font};
use asset;

use moho::input;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

use std::time::Duration;

pub struct PlayerSelect {
    collect: Collect,
    avoid: Avoid,
    gui: Gui,
}

impl PlayerSelect {
    pub fn new(animators: &Animators) -> Self {
        PlayerSelect {
            collect: Collect::new(animators),
            avoid: Avoid::new(animators),
            gui: Gui::new(animators),
        }
    }
}

impl World for PlayerSelect {
    type Quit = PlayerKind;

    fn update(self, input: &input::State, elapsed: Duration) -> game::State<Self> {
        let avoid = self.avoid;
        let collect = self.collect;

        self.gui.update(input, elapsed).map(|gui| {
            let collect = collect.update(input, elapsed).get();
            let avoid = avoid.update(input, elapsed).get();
            PlayerSelect {
                gui,
                avoid,
                collect,
            }
        })
    }
}

pub struct Assets<T> {
    title: Image<T>,
    instructions: Image<T>,
    avoid: avoid::Assets<T>,
    collect: collect::Assets<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> NextScene<PlayerSelect, (), ()> for Assets<T> {
    fn next(mut self, world: &PlayerSelect, _: &(), _: &mut ()) -> Result<Self> {
        self.avoid = self.avoid.next(&world.avoid, &(), &mut ())?;
        self.collect = self.collect.next(&world.collect, &(), &mut ())?;
        self.gui = self.gui.next(&world.gui, &(), &mut ())?;
        Ok(self)
    }
}

impl<T: Texture> Assets<T> {
    pub fn load<FM, AM>(
        font_manager: &mut FM,
        asset_manager: &mut AM,
        data: &data::Game,
    ) -> Result<Self>
    where
        FM: font::Manager<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        let font = font_manager.load(font::Kind::KenPixel, 64)?;
        let title = font.texturize("Select Player", &color)?
            .at(align::top(50).center(640));

        let collect = collect::Assets::load(&*font, asset_manager, data)?;
        let avoid = avoid::Assets::load(&*font, asset_manager, data)?;

        let instructions = {
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let text = "<Use Arrow Keys to choose player; then press Enter>";
            let height = font.measure(text)?.y as i32;
            font.texturize(text, &color)?
                .at(align::bottom(720 - height).center(640))
        };

        let gui = gui::Assets::load(asset_manager, data)?;

        Ok(Assets {
            title,
            collect,
            avoid,
            instructions,
            gui,
        })
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.collect)?;
        renderer.show(&self.avoid)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}
