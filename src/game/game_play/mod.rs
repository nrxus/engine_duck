use game;

use moho::{self, input};
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::renderer::Show;

use std::time::Duration;

pub struct GamePlay {}

impl GamePlay {
    pub fn new() -> Self {
        GamePlay {}
    }
}

impl World for GamePlay {
    type Quit = ();

    fn update(self, _: &input::State, _: Duration) -> game::State<Self> {
        moho::State::Running(self)
    }
}

pub struct Assets {}

impl NextScene<GamePlay, (), ()> for Assets {
    fn next(self, _: &GamePlay, _: &(), _: &mut ()) -> Result<Self> {
        Ok(self)
    }
}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Assets {})
    }
}

impl<R> Show<R> for Assets {
    fn show(&self, _: &mut R) -> Result<()> {
        Ok(())
    }
}
