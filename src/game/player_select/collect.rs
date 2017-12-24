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

pub struct Collect {
    gem: Animator,
    coin: Animator,
}

impl Collect {
    pub fn new(animators: &Animators) -> Self {
        Collect {
            gem: animators.gem.start(),
            coin: animators.coin.start(),
        }
    }
}

impl World for Collect {
    type Quit = moho::Never;

    fn update(mut self, _: &input::State, elapsed: Duration) -> game::State<Self> {
        self.coin.animate(elapsed);
        self.gem.animate(elapsed);
        moho::State::Running(self)
    }
}

pub struct Assets<T> {
    title: Image<T>,
    gem: Sprite<T>,
    coin: Sprite<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<F, AM>(font: &F, asset_manager: &mut AM, data: &data::Game) -> Result<Self>
    where
        F: Font<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);
        let title = font.texturize("Collect", &color)?
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
        Ok(Assets { title, gem, coin })
    }
}

impl<T> NextScene<Collect, (), ()> for Assets<T> {
    fn next(mut self, world: &Collect, _: &(), _: &mut ()) -> Result<Self> {
        self.gem.tile = world.gem.frame();
        self.coin.tile = world.coin.frame();
        Ok(self)
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.gem)?;
        renderer.show(&self.coin)
    }
}
