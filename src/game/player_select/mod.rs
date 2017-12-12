mod gui;

use self::gui::Gui;
use data::{self, Animators};
use game::font;
use asset::{self, Sprite};

use moho::{self, input};
use moho::animation::animator::Animator;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};

use std::time::Duration;

pub struct PlayerSelect {
    gem: Animator,
    coin: Animator,
    cat: Animator,
    gui: Gui,
}

impl PlayerSelect {
    pub fn new(animators: &Animators) -> Self {
        PlayerSelect {
            gem: animators.gem.start(),
            coin: animators.coin.start(),
            cat: animators.cat_idle.start(),
            gui: Gui::new(animators),
        }
    }
}

impl World for PlayerSelect {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        let mut gem = self.gem;
        let mut coin = self.coin;
        let mut cat = self.cat;

        self.gui.update(input, elapsed).map(|gui| {
            gem.animate(elapsed);
            coin.animate(elapsed);
            cat.animate(elapsed);

            PlayerSelect {
                gui,
                cat,
                coin,
                gem,
            }
        })
    }
}

pub struct Assets<T> {
    title: Image<T>,
    collect: Image<T>,
    avoid: Image<T>,
    instructions: Image<T>,
    gem: Sprite<T>,
    coin: Sprite<T>,
    cat: Sprite<T>,
    gui: gui::Assets<T>,
}

impl<T> NextScene<PlayerSelect, (), ()> for Assets<T> {
    fn next(mut self, world: &PlayerSelect, _: &(), _: &mut ()) -> Result<Self> {
        self.gem.tile = world.gem.frame();
        self.coin.tile = world.coin.frame();
        self.cat.tile = world.cat.frame();
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
        let collect = font.texturize("Collect", &color)?
            .at(align::top(400).center(960));
        let collect_distance = 50;
        let coin = {
            let data = &data.coin;
            let sheet = asset_manager.animation(&data.animation)?;
            let pos = align::top(525).right(320 - collect_distance / 2);
            let dst = data.out_size.dst(pos).scale(2);
            Sprite::new(sheet, dst)
        };
        let gem = {
            let data = &data.gem;
            let sheet = asset_manager.animation(&data.animation)?;
            let pos = align::top(525).left(320 + collect_distance / 2);
            let dst = data.out_size.dst(pos).scale(2);
            Sprite::new(sheet, dst)
        };

        // Avoid
        let avoid = font.texturize("Avoid", &color)?
            .at(align::top(400).center(320));
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
            gem,
            coin,
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
        renderer.show(&self.gem)?;
        renderer.show(&self.coin)?;
        renderer.show(&self.cat)?;
        renderer.show(&self.gui)
    }
}
