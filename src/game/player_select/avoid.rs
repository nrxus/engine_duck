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

pub struct Avoid {
    cat: Animator,
}

impl Avoid {
    pub fn new(animators: &Animators) -> Self {
        Avoid {
            cat: animators.cat_idle.start(),
        }
    }
}

impl World for Avoid {
    type Quit = moho::Never;

    fn update(mut self, _: &input::State, elapsed: Duration) -> game::State<Self> {
        self.cat.animate(elapsed);
        moho::State::Running(self)
    }
}

pub struct Assets<T> {
    title: Image<T>,
    cat: Sprite<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<F, AM>(font: &F, asset_manager: &mut AM, data: &data::Game) -> Result<Self>
    where
        F: Font<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);
        let title = font.texturize("Avoid", &color)?
            .at(align::top(400).center(960));
        let cat = {
            let data = &data.cat;
            let sheet = asset_manager.animation(&data.idle)?;
            let dst = data.out_size.dst(align::top(500).center(960)).scale(2);
            Sprite::new(sheet, dst)
        };
        Ok(Assets { title, cat })
    }
}

impl<T> NextScene<Avoid, (), ()> for Assets<T> {
    fn next(mut self, world: &Avoid, _: &(), _: &mut ()) -> Result<Self> {
        self.cat.tile = world.cat.frame();
        Ok(self)
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.cat)
    }
}
