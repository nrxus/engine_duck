use {asset, game};
use game::text::{self, Text};

use moho::{self, input};
use moho::font::Font;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::renderer::{align, options, Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;
use std::rc::Rc;

pub struct Hud {
    timer: Duration,
}

impl Default for Hud {
    fn default() -> Self {
        Hud {
            timer: Duration::from_secs(3),
        }
    }
}

impl World for Hud {
    type Quit = ();

    fn update(self, _: &input::State, elapsed: Duration) -> game::State<Self> {
        match self.timer.checked_sub(elapsed) {
            Some(timer) => moho::State::Running(Hud { timer }),
            None => moho::State::Quit(()),
        }
    }
}

impl text::Cached for Duration {
    type Value = u64;
    fn cached(&self) -> u64 {
        self.as_secs()
    }
}

pub struct Assets<T, F> {
    timer: Text<T, F, Duration>,
}

impl<T, F: Font<Texture = T>> NextScene<Hud, (), ()> for Assets<T, F> {
    fn next(mut self, hud: &Hud, _: &(), _: &mut ()) -> Result<Self> {
        self.timer.update(hud.timer)?;
        Ok(self)
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load<AM>(world: &Hud, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        let font = asset_manager.font(asset::Font::KenPixel, 32)?;
        Ok(Assets {
            timer: Text::load(world.timer, Rc::clone(&font), |v| format!("Time: {:03}", v))?,
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(
            &self.timer,
            options::at(align::top(0).center(960).dims(self.timer.dims())),
        )
    }
}
