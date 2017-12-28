use glm;
use moho::font::Font;
use moho::errors::*;
use moho::renderer::{self, ColorRGBA, Draw, Renderer, Show};
use moho::texture::Texture;

use std::rc::Rc;
use std::fmt::Debug;

struct CacheValue<T>(T);
pub trait Cached {
    type Value: PartialEq;
    fn cached(&self) -> Self::Value;
}

impl<T: Cached> PartialEq for CacheValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.cached() == other.0.cached()
    }
}

pub struct Text<T, F, V: Cached> {
    value: CacheValue<V>,
    texture: T,
    pattern: Box<Fn(V::Value) -> String>,
    font: Rc<F>,
}

impl<T, F: Font<Texture = T>, V: Cached + Debug> Text<T, F, V> {
    const COLOR: ColorRGBA = ColorRGBA(255, 255, 0, 255);

    pub fn load<P>(value: V, font: Rc<F>, pattern: P) -> Result<Self>
    where
        P: Fn(V::Value) -> String + 'static,
    {
        let text = pattern(value.cached());
        let texture = font.texturize(&text, &Self::COLOR)?;
        Ok(Text {
            value: CacheValue(value),
            pattern: Box::new(pattern),
            texture,
            font,
        })
    }

    pub fn update(&mut self, value: V) -> Result<()> {
        if value.cached() != self.value.0.cached() {
            let text = (self.pattern)(value.cached());
            self.texture = self.font.texturize(&text, &Self::COLOR)?;
            self.value = CacheValue(value);
        }
        Ok(())
    }
}

impl<T: Texture, F, V: Cached> Text<T, F, V> {
    pub fn dims(&self) -> glm::UVec2 {
        self.texture.dims()
    }
}

impl<R: Renderer, T: Draw<R>, F, V: Cached> Show<R> for Text<T, F, V> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.texture)
    }
}

impl<R: Renderer, T: Draw<R>, F, V: Cached> Draw<R> for Text<T, F, V> {
    fn draw(&self, options: renderer::Options, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.texture, options)
    }
}
