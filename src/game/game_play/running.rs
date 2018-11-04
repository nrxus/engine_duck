use crate::{
    asset,
    data::Animators,
    game::{
        hud::{self, Hud},
        player::{self, Player},
    },
    Result,
};

use moho::{self, engine::step::fixed, font::Font, input, texture::Texture};

use std::time::Duration;

pub struct Running {
    hud: Hud,
    player: Player,
}

impl Running {
    pub fn new(kind: player::Kind, animators: &Animators) -> Self {
        Running {
            hud: Hud::default(),
            player: Player::new(kind, animators),
        }
    }

    pub fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        let mut player = self.player;
        player.update(input, elapsed);

        self.hud
            .update(0, elapsed)
            .map(|hud| Running { hud, player })
    }
}

#[derive(moho::Show)]
pub struct Assets<T, F> {
    hud: hud::Assets<T, F>,
    player: player::Assets<T>,
}

impl<T, F: Font<Texture = T>> Assets<T, F> {
    pub fn next(mut self, world: &Running, _: &fixed::State) -> Result<Self> {
        self.hud = self.hud.next(&world.hud)?;
        self.player = self.player.next(&world.player);
        Ok(self)
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load(
        world: &Running,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        Ok(Assets {
            hud: hud::Assets::load(&world.hud, asset_manager)?,
            player: player::Assets::load(&world.player, asset_manager)?,
        })
    }
}
