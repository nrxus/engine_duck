use RefSnapshot;
use errors::*;
use moho::{self, input};
use moho::engine::{self, Engine, NextScene};
use moho::engine::step::fixed;
use moho::renderer::{self, Renderer};

use std::time::Duration;

pub fn run<'t, E, C>(engine: &mut Engine<E, C, fixed::FixedUpdate>) -> Result<()>
where
    E: input::EventPump,
    C: renderer::Canvas<'t>,
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
    fn update(self, _: &input::State, _: Duration) -> engine::State<Self> {
        engine::State::Running(self)
    }
}

pub struct Helper {}

pub struct Assets {}

impl NextScene<World, fixed::State, Helper> for Assets {
    fn next(self, _: RefSnapshot<World>, _: &mut Helper) -> moho::errors::Result<Self> {
        Ok(Assets {})
    }
}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for Assets {
    fn show(&self, _: &mut R) -> moho::errors::Result<()> {
        Ok(())
    }
}
