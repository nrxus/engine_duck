use Result;
use asset::{self, Sprite};
use data::Animators;
pub use game::player_select::PlayerKind as Kind;

use moho::animation::TileSheet;
use moho::animation::animator::{self, Animator};
use moho::input;
use moho::renderer::{align, options, Draw, Renderer, Show};
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

enum Action {
    Idle {
        animator: animator::Data,
    },
    Walk {
        direction: Direction,
        animator: Animator,
    },
    Jump {
        direction: Option<Direction>,
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
        let direction = {
            let left = input.is_key_down(Keycode::Left);
            let right = input.is_key_down(Keycode::Right);
            if left && !right {
                Some(Direction::Left)
            } else if right && !left {
                Some(Direction::Right)
            } else {
                None
            }
        };
        let up = input.is_key_down(Keycode::Space);
        self.action = match self.action {
            Action::Idle { animator } | Action::Jump { animator, .. } => if up {
                Action::Jump {
                    direction,
                    animator,
                }
            } else {
                match direction {
                    Some(direction) => Action::Walk {
                        direction,
                        animator: animator.start(),
                    },
                    None => Action::Idle { animator },
                }
            },
            Action::Walk { mut animator, .. } => if up {
                Action::Jump {
                    direction,
                    animator: animator.stop(),
                }
            } else {
                match direction {
                    Some(direction) => {
                        animator.animate(elapsed);
                        Action::Walk {
                            direction,
                            animator,
                        }
                    }
                    None => Action::Idle {
                        animator: animator.stop(),
                    },
                }
            },
        }
    }
}

pub enum Assets<T> {
    Idle(Image<T>, TileSheet<T>, Option<options::Flip>),
    Animated(Sprite<T>, Rc<T>, Option<options::Flip>),
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
        Ok(Assets::Idle(image, sheet, None))
    }
}

impl<T> Assets<T> {
    pub fn next(self, player: &Player) -> Self {
        match player.action {
            Action::Idle { .. } => match self {
                Assets::Animated(s, texture, flip) => Assets::Idle(
                    Image {
                        texture,
                        dst: s.dst,
                    },
                    s.sheet,
                    flip,
                ),
                b => b,
            },
            Action::Jump { direction, .. } => {
                let (image, sheet, f) = match self {
                    Assets::Animated(s, texture, f) => (
                        Image {
                            texture,
                            dst: s.dst,
                        },
                        s.sheet,
                        f,
                    ),
                    Assets::Idle(i, t, f) => (i, t, f),
                };
                let flip = direction
                    .map(|d| match d {
                        Direction::Left => Some(options::Flip::Horizontal),
                        Direction::Right => None,
                    })
                    .unwrap_or(f);
                Assets::Idle(image, sheet, flip)
            }
            Action::Walk {
                ref animator,
                direction,
            } => {
                let tile = animator.frame();
                let flip = match direction {
                    Direction::Left => Some(options::Flip::Horizontal),
                    Direction::Right => None,
                };
                let (sprite, texture) = match self {
                    Assets::Animated(mut sprite, texture, _) => {
                        sprite.tile = tile;
                        (sprite, texture)
                    }
                    Assets::Idle(i, sheet, _) => (
                        Sprite {
                            sheet,
                            tile,
                            dst: i.dst,
                        },
                        i.texture,
                    ),
                };
                Assets::Animated(sprite, texture, flip)
            }
        }
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Idle(ref asset, _, flip) => match flip {
                Some(f) => renderer.draw(asset, options::flip(f)),
                None => renderer.show(asset),
            },
            Assets::Animated(ref asset, _, flip) => match flip {
                Some(f) => renderer.draw(asset, options::flip(f)),
                None => renderer.show(asset),
            },
        }
    }
}
