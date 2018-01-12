mod action;

use Result;
use asset::{self, Sprite};
use data::Animators;
pub use game::player_select::PlayerKind as Kind;
pub use self::action::{Action, Direction};

use moho::input;
use moho::renderer::{align, options, Draw, Renderer, Show};
use moho::texture::{Image, Texture};

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

impl<T> Assets<T> {
    pub fn next(self, player: &Player) -> Self {
        use self::action::Assets::*;

        match player.action {
            Action::Idle { .. } => Assets {
                action: match self.action {
                    Animated(s, texture) => Idle(
                        Image {
                            texture,
                            dst: s.dst,
                        },
                        s.sheet,
                    ),
                    b => b,
                },
                flip: self.flip,
            },
            Action::Jump { direction, .. } => {
                let (image, sheet) = match self.action {
                    Animated(s, texture) => (
                        Image {
                            texture,
                            dst: s.dst,
                        },
                        s.sheet,
                    ),
                    Idle(i, t) => (i, t),
                };
                let action = Idle(image, sheet);
                let flip = direction
                    .map(Option::<options::Flip>::from)
                    .unwrap_or(self.flip);
                Assets { action, flip }
            }
            Action::Walk {
                ref animator,
                direction,
            } => {
                let tile = animator.frame();
                let (sprite, texture) = match self.action {
                    Animated(mut sprite, texture) => {
                        sprite.tile = tile;
                        (sprite, texture)
                    }
                    Idle(i, sheet) => (
                        Sprite {
                            sheet,
                            tile,
                            dst: i.dst,
                        },
                        i.texture,
                    ),
                };
                let action = Animated(sprite, texture);
                let flip = direction.into();
                Assets { action, flip }
            }
        }
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match self.action {
            action::Assets::Idle(ref asset, _) => match self.flip {
                Some(f) => renderer.draw(asset, options::flip(f)),
                None => renderer.show(asset),
            },
            action::Assets::Animated(ref asset, _) => match self.flip {
                Some(f) => renderer.draw(asset, options::flip(f)),
                None => renderer.show(asset),
            },
        }
    }
}
