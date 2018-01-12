use moho::input;
use sdl2::keyboard::Keycode;

#[derive(Clone, Copy, Debug)]
pub enum HKey {
    Left,
    Right,
}

pub trait InputStateExt {
    fn hkey(&self) -> Option<HKey>;
}

impl InputStateExt for input::State {
    fn hkey(&self) -> Option<HKey> {
        let left = self.is_key_down(Keycode::Left);
        let right = self.is_key_down(Keycode::Right);
        match (left, right) {
            (true, false) => Some(HKey::Left),
            (false, true) => Some(HKey::Right),
            _ => None,
        }
    }
}
