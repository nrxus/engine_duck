use {asset, Result};
pub use game::player_select::PlayerKind as Kind;

use moho::renderer::{Draw, Renderer, Show};
use moho::texture::Texture;

use std::rc::Rc;

pub struct Player {
    kind: Kind,
}

impl Player {
    pub fn new(kind: Kind) -> Self {
        Player { kind }
    }
}

pub struct Assets<T> {
    idle: Rc<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(player: &Player, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let player = match player.kind {
            Kind::Duck => asset::Texture::Duck,
            Kind::Husky => asset::Texture::Husky,
        };
        Ok(Assets {
            idle: asset_manager.texture(player)?,
        })
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&*self.idle)
    }
}
