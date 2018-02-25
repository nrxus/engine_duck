use Result;

use moho::{self, input};
use moho::engine::{self, Engine, NextScene};
use moho::engine::step::fixed;
use moho::renderer;

use std::time::Duration;

pub fn run<E, C>(engine: &mut Engine<E, C, fixed::FixedUpdate>) -> Result<()>
where
    E: input::EventPump,
    C: renderer::Canvas,
{
    let world = World {};
    let helper = Helper {};
    let assets = Assets {};
    engine
        .run::<Assets, _, _>(world, assets, helper)
        .map_err(Into::into)
}

pub struct World {}

impl engine::World for World {
    type Quit = ();

    fn update(self, _: &input::State, _: Duration) -> moho::State<Self, ()> {
        moho::State::Running(self)
    }
}

pub struct Helper {}

#[derive(Show)]
pub struct Assets {}

impl NextScene<World, fixed::State, Helper> for Assets {
    fn next(self, _: &World, _: &fixed::State, _: &mut Helper) -> Result<Self> {
        Ok(Assets {})
    }
}
