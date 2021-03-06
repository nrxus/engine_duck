use crate::{
    asset::{self, Animation, Font, Sprite, Texture},
    data, Result,
};

use moho::{animation::TileSheet, font, renderer::Position, texture, texture::Image};

use std::rc::Rc;

pub struct Helper<'t, 'f, TL, FL>
where
    TL: texture::Loader<'t>,
    FL: font::Loader<'f>,
{
    pub texture_manager: texture::Manager<'t, TL>,
    pub font_manager: font::Manager<'f, FL>,
    pub data: data::Game,
}

impl<'t, 'f, TL, FL> asset::Manager for Helper<'t, 'f, TL, FL>
where
    TL: texture::Loader<'t>,
    TL::Texture: texture::Texture,
    FL: font::Loader<'f>,
    FL::Font: font::Font<Texture = Rc<TL::Texture>>,
{
    type Texture = Rc<TL::Texture>;
    type Font = FL::Font;

    fn texture(&mut self, asset: Texture) -> Result<Self::Texture> {
        let data = self.data.texture(asset);
        self.texture_manager
            .load(&format!("media/sprites/{}", data.texture))
            .map_err(Into::into)
    }

    fn image(&mut self, asset: Texture, pos: Position) -> Result<Image<Self::Texture>> {
        let dims = self.data.texture(asset).dims;

        self.texture(asset).map(|texture| {
            let dst = dims.at(pos);
            Image { texture, dst }
        })
    }

    fn sheet(&mut self, animation: Animation) -> Result<TileSheet<Self::Texture>> {
        let data = self.data.animation(animation);
        self.texture_manager
            .load(&format!("media/sprites/{}", data.texture))
            .map(|t| TileSheet::new(data.tiles.into(), t))
            .map_err(Into::into)
    }

    fn sprite(&mut self, asset: Animation, pos: Position) -> Result<Sprite<Self::Texture>> {
        let dims = self.data.animation(asset).dims;

        self.sheet(asset).map(|s| {
            let dst = dims.at(pos);
            Sprite::new(s, dst)
        })
    }

    fn font(&mut self, font: Font, size: u16) -> Result<Rc<Self::Font>> {
        self.font_manager
            .load(&font::Details {
                path: font.path(),
                size,
            })
            .map_err(Into::into)
    }
}
