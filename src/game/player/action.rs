use asset::Sprite;
use utils::InputStateExt;
use utils::HKey as Direction;

use moho::animation::animator::{self, Animator};
use moho::animation::TileSheet;
use moho::input;
use moho::texture::Image;
use sdl2::keyboard::Keycode;

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
        match (input.is_key_down(Keycode::Space), input.hkey()) {
            (true, direction) => Action::Jump {
                animator: self.stopped_animator(),
                direction,
            },
            (false, None) => Action::Idle {
                animator: self.stopped_animator(),
            },
            (false, Some(direction)) => Action::Walk {
                animator: self.running_animator(elapsed),
                direction,
            },
        }
    }

    fn stopped_animator(self) -> animator::Data {
        match self {
            Action::Idle { animator } | Action::Jump { animator, .. } => animator,
            Action::Walk { animator, .. } => animator.stop(),
        }
    }

    fn running_animator(self, elapsed: Duration) -> Animator {
        match self {
            Action::Idle { animator } | Action::Jump { animator, .. } => animator.start(),
            Action::Walk { mut animator, .. } => {
                animator.animate(elapsed);
                animator
            }
        }
    }
}

pub enum Assets<T> {
    Idle(Image<T>, TileSheet<T>),
    Animated(Sprite<T>, T),
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
