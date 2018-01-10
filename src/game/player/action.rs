use moho::animation::animator::{self, Animator};
use moho::input;
use sdl2::keyboard::Keycode;

use std::time::Duration;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

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
