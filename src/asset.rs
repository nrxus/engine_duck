use data;

use glm;
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
    fn load_texture(&mut self, texture: &data::Texture) -> Result<Rc<T>>;
    fn load_image(
        &mut self,
        image: &data::Image,
        pos: options::Position,
        scale: u32,
    ) -> Result<Image<T>>;
    fn load_player_image(
        &mut self,
        player: &data::Player,
        pos: options::Position,
        scale: u32,
    ) -> Result<Image<T>>;
    fn load_sprite(&mut self, sprite: &data::Sprite) -> Result<animation::Data<T>>;
}

impl<'t, TL> AssetLoader<TL::Texture> for TextureManager<'t, TL>
where
    TL: TextureLoader<'t>,
    TL::Texture: Texture,
{
    fn load_texture(&mut self, texture: &data::Texture) -> Result<Rc<TL::Texture>> {
        self.load(&format!("media/sprites/{}", texture.0))
    }

    fn load_image(
        &mut self,
        image: &data::Image,
        pos: options::Position,
        scale: u32,
    ) -> Result<Image<TL::Texture>> {
        let texture = self.load_texture(&image.texture)?;
        let dims = glm::UVec2::from(image.out_size) * scale;
        let dst = pos.dims(dims);
        Ok(Image { texture, dst })
    }

    fn load_player_image(
        &mut self,
        player: &data::Player,
        pos: options::Position,
        scale: u32,
    ) -> Result<Image<TL::Texture>> {
        let texture = self.load_texture(&player.idle_texture)?;
        let dims = glm::UVec2::from(player.out_size) * scale;
        let dst = pos.dims(dims);
        Ok(Image { texture, dst })
    }

    fn load_sprite(&mut self, sprite: &data::Sprite) -> Result<animation::Data<TL::Texture>> {
        let texture = self.load_texture(&sprite.texture)?;
        let sheet = TileSheet::new(sprite.tiles.into(), texture);
        let duration = Duration::from_millis(sprite.duration / u64::from(sprite.frames));
        let animator = animator::Data::new(sprite.frames, duration);
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
