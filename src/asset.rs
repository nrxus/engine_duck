use {data, Result};

use moho::animation::TileSheet;
use moho::renderer::options::{self, Options};
use moho::renderer::{Destination, Draw, Position, Renderer, Show};
use moho::texture::{self, Image};
use moho::font;

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

    pub fn scale(mut self, scale: u32) -> Self {
        self.dst = self.dst.scale(scale);
        self
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
    type Texture: texture::Texture;
    type Font: font::Font<Texture = Self::Texture>;

    fn texture(&mut self, texture: Texture) -> Result<Self::Texture>;
    fn sheet(&mut self, animation: Animation) -> Result<TileSheet<Self::Texture>>;
    fn image(&mut self, texture: Texture, pos: Position) -> Result<Image<Self::Texture>>;
    fn sprite(&mut self, animaiton: Animation, pos: Position) -> Result<Sprite<Self::Texture>>;
    fn font(&mut self, font: Font, size: u16) -> Result<Rc<Self::Font>>;
}

#[derive(Clone, Copy)]
pub enum Texture {
    Husky,
    Duck,
    Heart,
}

#[derive(Clone, Copy)]
pub enum Animation {
    Husky,
    Duck,
    Coin,
    Gem,
    IdleCat,
}

#[derive(Clone, Copy)]
pub enum Font {
    KenPixel,
    Joystix,
}

impl Font {
    pub fn path(&self) -> &'static str {
        match *self {
            Font::KenPixel => "media/fonts/kenpixel_mini.ttf",
            Font::Joystix => "media/fonts/joystix.monospace.ttf",
        }
    }
}

pub struct TextureData<'t> {
    pub texture: &'t str,
    pub dims: data::Dimension,
}

pub struct AnimationData<'t> {
    pub texture: &'t str,
    pub dims: data::Dimension,
    pub tiles: data::Dimension,
}

impl data::Game {
    pub fn texture<'t>(&'t self, texture: Texture) -> TextureData<'t> {
        match texture {
            Texture::Husky => TextureData {
                texture: &self.husky.idle_texture.0,
                dims: self.husky.out_size,
            },
            Texture::Duck => TextureData {
                texture: &self.duck.idle_texture.0,
                dims: self.duck.out_size,
            },
            Texture::Heart => TextureData {
                texture: &self.heart.texture.0,
                dims: self.heart.out_size,
            },
        }
    }

    pub fn animation<'t>(&'t self, animation: Animation) -> AnimationData<'t> {
        let (dims, animation) = match animation {
            Animation::Husky => (self.husky.out_size, &self.husky.animation),
            Animation::Duck => (self.duck.out_size, &self.duck.animation),
            Animation::Coin => (self.coin.out_size, &self.coin.animation),
            Animation::Gem => (self.gem.out_size, &self.gem.animation),
            Animation::IdleCat => (self.cat.out_size, &self.cat.idle),
        };
        AnimationData {
            dims,
            texture: &animation.texture.0,
            tiles: animation.tiles,
        }
    }
}
