use asset::{self, Sprite};
use data::{self, Animators};
use game;

use moho::{self, input};
use moho::animation::animator::Animator;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};
use moho::texture::{Image, Texture};

use std::time::Duration;

pub struct Guide {
    gem: Animator,
    coin: Animator,
    cat: Animator,
}

impl Guide {
    pub fn new(animators: &Animators) -> Self {
        Guide {
            gem: animators.gem.start(),
            coin: animators.coin.start(),
            cat: animators.cat_idle.start(),
        }
    }
}

impl World for Guide {
    type Quit = moho::Never;

    fn update(mut self, _: &input::State, elapsed: Duration) -> game::State<Self> {
        self.coin.animate(elapsed);
        self.gem.animate(elapsed);
        self.cat.animate(elapsed);
        moho::State::Running(self)
    }
}

pub struct Assets<T> {
    collect: Image<T>,
    avoid: Image<T>,
    gem: Sprite<T>,
    coin: Sprite<T>,
    cat: Sprite<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<F, AM>(font: &F, asset_manager: &mut AM, data: &data::Game) -> Result<Self>
    where
        F: Font<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        //Collect
        let collect = font.texturize("Collect", &color)?
            .at(align::top(400).center(320));
        let distance = 50;
        let coin = {
            let data = &data.coin;
            let sheet = asset_manager.animation(&data.animation)?;
            let pos = align::top(525).right(320 - distance / 2);
            let dst = data.out_size.dst(pos).scale(2);
            Sprite::new(sheet, dst)
        };
        let gem = {
            let data = &data.gem;
            let sheet = asset_manager.animation(&data.animation)?;
            let pos = align::top(525).left(320 + distance / 2);
            let dst = data.out_size.dst(pos).scale(2);
            Sprite::new(sheet, dst)
        };

        //Avoid
        let avoid = font.texturize("Avoid", &color)?
            .at(align::top(400).center(960));
        let cat = {
            let data = &data.cat;
            let sheet = asset_manager.animation(&data.idle)?;
            let dst = data.out_size.dst(align::top(500).center(960)).scale(2);
            Sprite::new(sheet, dst)
        };
        Ok(Assets {
            collect,
            gem,
            coin,
            avoid,
            cat,
        })
    }
}

impl<T> NextScene<Guide, (), ()> for Assets<T> {
    fn next(mut self, world: &Guide, _: &(), _: &mut ()) -> Result<Self> {
        self.gem.tile = world.gem.frame();
        self.coin.tile = world.coin.frame();
        self.cat.tile = world.cat.frame();
        Ok(self)
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.collect)?;
        renderer.show(&self.gem)?;
        renderer.show(&self.coin)?;
        renderer.show(&self.avoid)?;
        renderer.show(&self.cat)
    }
}
