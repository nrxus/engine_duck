use data;

use moho::animation::{self, animator, TileSheet};
use moho::errors::*;
use moho::renderer::{options, Renderer, Scene, Texture, TextureLoader, TextureManager};

use std::rc::Rc;
use std::time::Duration;

pub struct Image<T> {
    pub texture: Rc<T>,
    pub dst: options::Destination,
}

pub struct Sprite<T> {
    pub animation: animation::Data<T>,
    pub dst: options::Destination,
}

pub trait AssetLoader<T> {
    fn texture(&mut self, texture: &data::Texture) -> Result<Rc<T>>;
    fn animation(&mut self, animation: &data::Animation) -> Result<animation::Data<T>>;
}

impl<'t, TL> AssetLoader<TL::Texture> for TextureManager<'t, TL>
where
    TL: TextureLoader<'t>,
    TL::Texture: Texture,
{
    fn texture(&mut self, texture: &data::Texture) -> Result<Rc<TL::Texture>> {
        self.load(&format!("media/sprites/{}", texture.0))
    }

    fn animation(&mut self, animation: &data::Animation) -> Result<animation::Data<TL::Texture>> {
        let texture = self.texture(&animation.texture)?;
        let sheet = TileSheet::new(animation.tiles.into(), texture);
        let duration = Duration::from_millis(animation.duration / u64::from(animation.frames));
        let animator = animator::Data::new(animation.frames, duration);
        Ok(animation::Data::new(animator, sheet))
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Image<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.copy(&*self.texture, options::at(self.dst))
    }
}
