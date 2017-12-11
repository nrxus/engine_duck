use data;

use moho::animation::TileSheet;
use moho::errors::*;
use moho::renderer::options::{self, Destination, Options};
use moho::renderer::{Draw, Renderer, Show};
use moho::texture::{self, Texture};

use std::rc::Rc;

pub struct Sprite<T> {
    pub sheet: TileSheet<T>,
    pub dst: Destination,
    pub tile: u32,
}

impl<T> Sprite<T> {
    pub fn new(sheet: TileSheet<T>, dst: Destination) -> Self {
        Sprite {
            sheet,
            dst,
            tile: 0,
        }
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Sprite<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.sheet.tile(self.tile), options::at(self.dst))
    }
}

impl<R: Renderer, T: Draw<R>> Draw<R> for Sprite<T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.sheet.tile(self.tile), options.at(self.dst))
    }
}

pub trait Manager {
    type Texture: Texture;

    fn texture(&mut self, texture: &data::Texture) -> Result<Rc<Self::Texture>>;
    fn animation(&mut self, animation: &data::Animation) -> Result<TileSheet<Self::Texture>>;
}

impl<'t, TL> Manager for texture::Manager<'t, TL>
where
    TL: texture::Loader<'t>,
    TL::Texture: Texture,
    Error: From<TL::Error>,
{
    type Texture = TL::Texture;

    fn texture(&mut self, texture: &data::Texture) -> Result<Rc<Self::Texture>> {
        self.load(&format!("media/sprites/{}", texture.0))
            .map_err(Into::into)
    }

    fn animation(&mut self, animation: &data::Animation) -> Result<TileSheet<Self::Texture>> {
        self.texture(&animation.texture)
            .map(|t| TileSheet::new(animation.tiles.into(), t))
    }
}
