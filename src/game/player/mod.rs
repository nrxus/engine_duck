use Result;
use asset::{self, Sprite};
use data::Animators;
pub use game::player_select::PlayerKind as Kind;

use glm;
use moho::animation::TileSheet;
use moho::animation::animator::{self, Animator};
use moho::input;
use moho::renderer::{align, Draw, Renderer, Show};
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

enum Action {
    Idle {
        animator: animator::Data,
    },
    Walk {
        velocity: f64,
        animator: Animator,
    },
    Jump {
        velocity: glm::DVec2,
        animator: animator::Data,
    },
}

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
        let x_vel = {
            let left = input.is_key_down(Keycode::Left);
            let right = input.is_key_down(Keycode::Right);
            if left && !right {
                -2
            } else if right && !left {
                2
            } else {
                0
            }
        };
        let up = input.is_key_down(Keycode::Space);
        self.action = match self.action {
            Action::Idle { animator } | Action::Jump { animator, .. } => if up {
                Action::Jump {
                    velocity: glm::dvec2(10., f64::from(x_vel)),
                    animator,
                }
            } else if x_vel != 0 {
                Action::Walk {
                    velocity: f64::from(x_vel),
                    animator: animator.start(),
                }
            } else {
                Action::Idle { animator }
            },
            Action::Walk { mut animator, .. } => if up {
                Action::Jump {
                    velocity: glm::dvec2(10., f64::from(x_vel)),
                    animator: animator.stop(),
                }
            } else if x_vel != 0 {
                animator.animate(elapsed);
                Action::Walk {
                    velocity: f64::from(x_vel),
                    animator,
                }
            } else {
                Action::Idle {
                    animator: animator.stop(),
                }
            },
        }
    }
}

pub enum Assets<T> {
    Idle(Image<T>, TileSheet<T>),
    Animated(Sprite<T>, Rc<T>),
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
        Ok(Assets::Idle(image, sheet))
    }
}

impl<T> Assets<T> {
    pub fn next(self, player: &Player) -> Self {
        match player.action {
            Action::Idle { .. } | Action::Jump { .. } => {
                let (image, sheet) = match self {
                    Assets::Animated(s, texture) => (
                        Image {
                            texture,
                            dst: s.dst,
                        },
                        s.sheet,
                    ),
                    Assets::Idle(i, t) => (i, t),
                };
                Assets::Idle(image, sheet)
            }
            Action::Walk { ref animator, .. } => {
                let tile = animator.frame();
                let (sprite, texture) = match self {
                    Assets::Animated(mut sprite, texture) => {
                        sprite.tile = tile;
                        (sprite, texture)
                    }
                    Assets::Idle(i, sheet) => (
                        Sprite {
                            sheet,
                            tile,
                            dst: i.dst,
                        },
                        i.texture,
                    ),
                };
                Assets::Animated(sprite, texture)
            }
        }
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Idle(ref asset, _) => renderer.show(asset),
            Assets::Animated(ref asset, _) => renderer.show(asset),
        }
    }
}
