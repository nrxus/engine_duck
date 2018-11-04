mod running;
mod timeup;

use self::{running::Running, timeup::TimeUp};
use crate::{asset, data::Animators, game::player, Result};

use moho::{self, engine::step::fixed, font::Font, input, texture::Texture};

use std::time::Duration;

pub enum GamePlay {
    Running(Running),
    TimeUp(TimeUp),
}

impl GamePlay {
    pub fn new(kind: player::Kind, animators: &Animators) -> Self {
        GamePlay::Running(Running::new(kind, animators))
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

#[derive(moho::Show)]
pub enum Assets<T, F> {
    Running(running::Assets<T, F>),
    TimeUp(timeup::Assets<T, F>),
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn next(
        self,
        world: &GamePlay,
        step: &fixed::State,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        match *world {
            GamePlay::Running(ref world) => match self {
                Assets::Running(r) => r.next(world, step),
                _ => running::Assets::load(world, asset_manager),
            }
            .map(Assets::Running),
            GamePlay::TimeUp(_) => match self {
                Assets::TimeUp(t) => Ok(t),
                Assets::Running(r) => timeup::Assets::load(asset_manager, r),
            }
            .map(Assets::TimeUp),
        }
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load(
        world: &GamePlay,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        match *world {
            GamePlay::Running(ref world) => {
                running::Assets::load(world, asset_manager).map(Assets::Running)
            }
            _ => unreachable!("cannot load timeup state without a previous running state"),
        }
    }
}
