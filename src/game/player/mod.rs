mod action;

use Result;
use asset;
use data::Animators;
pub use game::player_select::PlayerKind as Kind;
pub use self::action::{Action, Direction};

use moho::input;
use moho::renderer::{align, options, Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;

pub struct Player {
    kind: Kind,
    action: Action,
}

impl Player {
    pub fn new(kind: Kind, animators: &Animators) -> Self {
        let animator = match kind {
            Kind::Husky => animators.husky,
            Kind::Duck => animators.duck,
        };
        Player {
            kind,
            action: Action::Idle { animator },
        }
    }

    pub fn update(&mut self, input: &input::State, elapsed: Duration) {
        self.action = self.action.update(input, elapsed);
    }
}

pub struct Assets<T> {
    action: action::Assets<T>,
    flip: Option<options::Flip>,
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(player: &Player, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let (texture, animation) = match player.kind {
            Kind::Duck => (asset::Texture::Duck, asset::Animation::Duck),
            Kind::Husky => (asset::Texture::Husky, asset::Animation::Husky),
        };
        let image = asset_manager.image(texture, align::left(0).top(200))?;
        let sheet = asset_manager.sheet(animation)?;
        let action = action::Assets::Idle(image, sheet);
        let flip = None;
        Ok(Assets { action, flip })
    }
}

impl From<Direction> for Option<options::Flip> {
    fn from(direction: Direction) -> Option<options::Flip> {
        match direction {
            Direction::Left => Some(options::Flip::Horizontal),
            Direction::Right => None,
        }
    }
}

impl<T> Assets<T> {
    pub fn next(self, player: &Player) -> Self {
        let action = self.action.next(&player.action);
        let flip = match player.action {
            Action::Idle { .. } => self.flip,
            Action::Jump { direction, .. } => direction
                .map(Option::<options::Flip>::from)
                .unwrap_or(self.flip),
            Action::Walk { direction, .. } => direction.into(),
        };

        Assets { action, flip }
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        let options = match self.flip {
            Some(f) => options::flip(f),
            None => options::none(),
        };

        match self.action {
            action::Assets::Idle(ref asset, _) => renderer.draw(asset, options),
            action::Assets::Animated(ref asset, _) => renderer.draw(asset, options),
        }
    }
}
