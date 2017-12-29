mod running;
mod timeup;

use self::running::Running;
use self::timeup::TimeUp;
use asset;

use moho::{self, input};
use moho::engine::step::fixed;
use moho::errors::*;
use moho::font::Font;
use moho::renderer::{Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;

pub enum GamePlay {
    Running(Running),
    TimeUp(TimeUp),
}

impl GamePlay {
    pub fn new() -> Self {
        GamePlay::Running(Running::new())
    }

    pub fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        match self {
            GamePlay::Running(r) => moho::State::Running(
                r.update(input, elapsed)
                    .map(GamePlay::Running)
                    .catch_quit(|_| GamePlay::TimeUp(TimeUp {})),
            ),
            GamePlay::TimeUp(t) => t.update(input).map(GamePlay::TimeUp),
        }
    }
}

pub enum Assets<T, F> {
    Running(running::Assets<T, F>),
    TimeUp(timeup::Assets<T, F>),
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn next<AM>(
        self,
        world: &GamePlay,
        step: &fixed::State,
        asset_manager: &mut AM,
    ) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        match *world {
            GamePlay::Running(ref world) => match self {
                Assets::Running(r) => r.next(world, step),
                _ => running::Assets::load(world, asset_manager),
            }.map(Assets::Running),
            GamePlay::TimeUp(_) => match self {
                Assets::TimeUp(t) => Ok(t),
                Assets::Running(r) => timeup::Assets::load(asset_manager, r),
            }.map(Assets::TimeUp),
        }
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load<AM>(world: &GamePlay, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        match *world {
            GamePlay::Running(ref world) => {
                running::Assets::load(world, asset_manager).map(Assets::Running)
            }
            _ => unreachable!("cannot load timeup state without a previous running state"),
        }
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Running(ref assets) => renderer.show(assets),
            Assets::TimeUp(ref assets) => renderer.show(assets),
        }
    }
}
