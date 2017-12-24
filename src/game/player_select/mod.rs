mod gui;
mod collect;

pub use self::gui::ButtonKind as PlayerKind;
use self::gui::Gui;
use self::collect::Collect;
use data::{self, Animators};
use game::{self, font};
use asset::{self, Sprite};

use moho::input;
use moho::animation::animator::Animator;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

use std::time::Duration;

pub struct PlayerSelect {
    collect: Collect,
    cat: Animator,
    gui: Gui,
}

impl PlayerSelect {
    pub fn new(animators: &Animators) -> Self {
        PlayerSelect {
            collect: Collect::new(animators),
            cat: animators.cat_idle.start(),
            gui: Gui::new(animators),
        }
    }
}

impl World for PlayerSelect {
    type Quit = PlayerKind;

    fn update(self, input: &input::State, elapsed: Duration) -> game::State<Self> {
        let mut cat = self.cat;
        let collect = self.collect;

        self.gui.update(input, elapsed).map(|gui| {
            cat.animate(elapsed);
            let collect = collect.update(input, elapsed).get();
            PlayerSelect { gui, cat, collect }
        })
    }
}

pub struct Assets<T> {
    title: Image<T>,
    avoid: Image<T>,
    instructions: Image<T>,
    cat: Sprite<T>,
    collect: collect::Assets<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> NextScene<PlayerSelect, (), ()> for Assets<T> {
    fn next(mut self, world: &PlayerSelect, _: &(), _: &mut ()) -> Result<Self> {
        self.cat.tile = world.cat.frame();
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

        // Collect
        let collect = collect::Assets::load(&*font, asset_manager, data)?;

        // Avoid
        let avoid = font.texturize("Avoid", &color)?
            .at(align::top(400).center(960));
        let cat = {
            let data = &data.cat;
            let sheet = asset_manager.animation(&data.idle)?;
            let pos = align::center(960).top(500);
            let dst = data.out_size.dst(pos).scale(2);
            Sprite::new(sheet, dst)
        };

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
            cat,
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
        renderer.show(&self.cat)?;
        renderer.show(&self.gui)
    }
}
