use {asset, game};
use game::hud::{self, Hud};

use moho::input;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::font::Font;
use moho::renderer::{Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;

pub struct GamePlay {
    hud: Hud,
}

impl GamePlay {
    pub fn new() -> Self {
        GamePlay {
            hud: Hud::default(),
        }
    }
}

impl World for GamePlay {
    type Quit = ();

    fn update(self, _: &input::State, elapsed: Duration) -> game::State<Self> {
        self.hud.update(0, elapsed).map(|hud| GamePlay { hud })
    }
}

pub struct Assets<T, F> {
    hud: hud::Assets<T, F>,
}

impl<T, F: Font<Texture = T>> NextScene<GamePlay, (), ()> for Assets<T, F> {
    fn next(mut self, world: &GamePlay, _: &(), _: &mut ()) -> Result<Self> {
        self.hud = self.hud.next(&world.hud, &(), &mut ())?;
        Ok(self)
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load<AM>(world: &GamePlay, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        Ok(Assets {
            hud: hud::Assets::load(&world.hud, asset_manager)?,
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.hud)
    }
}
