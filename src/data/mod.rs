mod level;

pub use self::level::{CatKind, GroundKind, Level, Obstacle};
use Result;

use glm;
use moho::animation::animator;
use moho::renderer::options::{Destination, Position};
use serde_yaml;

use std::fs::File;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Dimension {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Animation {
    pub texture: Texture,
    pub frames: u32,
    pub tiles: Dimension,
    pub duration: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Texture(pub String);

#[derive(Debug, Deserialize, Clone)]
pub enum Shape {
    Rectangle(Dimension, Dimension),
    Circle(Dimension, f64),
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub animation: Animation,
    pub idle_texture: Texture,
    pub out_size: Dimension,
    pub body: Vec<Shape>,
    pub legs: Vec<Shape>,
}

#[derive(Debug, Deserialize)]
pub struct Cat {
    pub idle: Animation,
    pub walking: Animation,
    pub out_size: Dimension,
    pub body: Vec<Shape>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub texture: Texture,
    pub out_size: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Collectable {
    pub animation: Animation,
    pub out_size: Dimension,
    pub score: u32,
}

#[derive(Debug, Deserialize)]
pub struct Ground {
    pub center: Texture,
    pub left: Texture,
    pub right: Texture,
    pub top: Texture,
    pub top_left: Texture,
    pub top_right: Texture,
    pub out_size: Dimension,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub duck: Player,
    pub husky: Player,
    pub ground: Ground,
    pub gem: Collectable,
    pub coin: Collectable,
    pub cat: Cat,
    pub background: Image,
    pub goal: Image,
    pub heart: Image,
    pub spike: Image,
}

impl Game {
    pub fn load(path: &'static str) -> Result<Game> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }

    pub fn animators(&self) -> Animators {
        Animators {
            duck: self.duck.animation.animator(),
            husky: self.husky.animation.animator(),
            gem: self.gem.animation.animator(),
            cat_idle: self.cat.idle.animator(),
            cat_walking: self.cat.walking.animator(),
            coin: self.coin.animation.animator(),
        }
    }
}

impl From<Dimension> for glm::UVec2 {
    fn from(dim: Dimension) -> glm::UVec2 {
        let Dimension { x, y } = dim;
        glm::uvec2(x, y)
    }
}

impl From<Dimension> for glm::IVec2 {
    fn from(dim: Dimension) -> glm::IVec2 {
        let Dimension { x, y } = dim;
        glm::ivec2(x as i32, y as i32)
    }
}

impl From<Dimension> for glm::DVec2 {
    fn from(dim: Dimension) -> glm::DVec2 {
        let Dimension { x, y } = dim;
        glm::dvec2(x.into(), y.into())
    }
}

impl Dimension {
    pub fn at(self, pos: Position) -> Destination {
        pos.dims(self.into())
    }
}

impl Animation {
    fn animator(&self) -> animator::Data {
        let duration = Duration::from_millis(self.duration / u64::from(self.frames));
        animator::Data::new(self.frames, duration)
    }
}

pub struct Animators {
    pub duck: animator::Data,
    pub husky: animator::Data,
    pub gem: animator::Data,
    pub cat_idle: animator::Data,
    pub cat_walking: animator::Data,
    pub coin: animator::Data,
}
