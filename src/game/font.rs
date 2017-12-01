use moho::errors::*;
use moho::font;

use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Kind {
    KenPixel,
    Joystix,
}

impl Kind {
    pub fn path(&self) -> &'static str {
        match *self {
            Kind::KenPixel => "media/fonts/kenpixel_mini.ttf",
            Kind::Joystix => "media/fonts/joystix.monospace.ttf",
        }
    }
}

pub trait Manager: Sized {
    type Font: font::Font<Texture = Self::Texture>;
    type Texture;

    fn load(&mut self, kind: Kind, size: u16) -> Result<Rc<Self::Font>>;
}

impl<'f, FL: font::Loader<'f>> Manager for font::Manager<'f, FL> {
    type Font = FL::Font;
    type Texture = <FL::Font as font::Font>::Texture;

    fn load(&mut self, kind: Kind, size: u16) -> Result<Rc<FL::Font>> {
        self.load(&font::Details {
            path: kind.path(),
            size,
        }).chain_err(|| format!("cannot load font in path: {:?}", kind.path()))
    }
}
