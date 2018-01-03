use {asset, Result};
use game::hud::{self, Hud};
use game::player::{self, Player};

use moho::{self, input};
use moho::engine::step::fixed;
use moho::font::Font;
use moho::renderer::{Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;

pub struct Running {
    hud: Hud,
    player: Player,
}

impl Running {
    pub fn new(kind: player::Kind) -> Self {
        Running {
            hud: Hud::default(),
            player: Player::new(kind),
        }
    }

    pub fn update(self, _: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        let player = self.player;

        self.hud
            .update(0, elapsed)
            .map(|hud| Running { hud, player })
    }
}

pub struct Assets<T, F> {
    hud: hud::Assets<T, F>,
    player: player::Assets<T>,
}

impl<T, F: Font<Texture = T>> Assets<T, F> {
    pub fn next(mut self, world: &Running, _: &fixed::State) -> Result<Self> {
        self.hud = self.hud.next(&world.hud)?;
        Ok(self)
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load<AM>(world: &Running, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        Ok(Assets {
            hud: hud::Assets::load(&world.hud, asset_manager)?,
            player: player::Assets::load(&world.player, asset_manager)?,
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.hud)?;
        renderer.show(&self.player)
    }
}
