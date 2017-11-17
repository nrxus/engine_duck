use moho::{self, input};
use moho::engine::{self, Engine};
use moho::engine::step::fixed;
use moho::renderer::{self, Renderer};

use std::time::Duration;

pub fn run<'t, E, C>(engine: &mut Engine<E, C, fixed::FixedUpdate>) -> moho::errors::Result<()>
where
    E: input::EventPump,
    C: renderer::Canvas<'t>,
{
    let world = World {};
    let helper = Helper {};
    engine.run::<Scene, _, _>(world, helper)
}

pub struct World {}

impl engine::World for World {
    fn update(self, input: &input::State, elapsed: Duration) -> engine::State<Self> {
        engine::State::Running(self)
    }
}

impl engine::IntoScene<Scene, fixed::State, Helper> for World {
    fn try_into(&self, step: &fixed::State, helper: &mut Helper) -> moho::errors::Result<Scene> {
        Ok(Scene {})
    }
}

pub struct Helper {}

pub struct Scene {}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for Scene {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        Ok(())
    }
}
