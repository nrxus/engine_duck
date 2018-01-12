use asset::Sprite;
use utils::InputStateExt;
use utils::HKey as Direction;

use moho::animation::animator::{self, Animator};
use moho::animation::TileSheet;
use moho::input;
use moho::texture::Image;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum Action {
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

impl Action {
    pub fn update(self, input: &input::State, elapsed: Duration) -> Self {
        use self::Action::*;

        let direction = input.hkey();
        let up = input.is_key_down(Keycode::Space);
        match self {
            Idle { animator } | Jump { animator, .. } => if up {
                Jump {
                    direction,
                    animator,
                }
            } else {
                match direction {
                    Some(direction) => Walk {
                        direction,
                        animator: animator.start(),
                    },
                    None => Idle { animator },
                }
            },
            Walk { mut animator, .. } => if up {
                Jump {
                    direction,
                    animator: animator.stop(),
                }
            } else {
                match direction {
                    Some(direction) => {
                        animator.animate(elapsed);
                        Walk {
                            direction,
                            animator,
                        }
                    }
                    None => Idle {
                        animator: animator.stop(),
                    },
                }
            },
        }
    }
}

pub enum Assets<T> {
    Idle(Image<T>, TileSheet<T>),
    Animated(Sprite<T>, Rc<T>),
}

impl<T> Assets<T> {
    pub fn next(self, action: &Action) -> Self {
        use self::Assets::*;

        match *action {
            Action::Idle { .. } | Action::Jump { .. } => match self {
                Animated(s, texture) => Idle(
                    Image {
                        texture,
                        dst: s.dst,
                    },
                    s.sheet,
                ),
                idle => idle,
            },
            Action::Walk { ref animator, .. } => {
                let tile = animator.frame();
                let (sprite, texture) = match self {
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
                Animated(sprite, texture)
            }
        }
    }
}
