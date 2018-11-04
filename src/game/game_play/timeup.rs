use super::running;
use crate::{asset, Result};

use moho::{
    self,
    font::Font,
    input,
    renderer::{align, ColorRGBA, Draw, Renderer, Show},
    texture::{Image, Texture},
};
use sdl2::keyboard::Keycode;

pub struct TimeUp {}

impl TimeUp {
    pub fn update(self, input: &input::State) -> moho::State<Self, ()> {
        if input.did_press_key(Keycode::Return) {
            moho::State::Quit(())
        } else {
            moho::State::Running(self)
        }
    }
}

pub struct Assets<T, F> {
    game: running::Assets<T, F>,
    alert: Image<T>,
    instructions: Image<T>,
}

impl<T: Texture, F> Assets<T, F> {
    pub fn load(
        asset_manager: &mut impl asset::Manager<Texture = T>,
        game: running::Assets<T, F>,
    ) -> Result<Self> {
        let font = asset_manager.font(asset::Font::KenPixel, 48)?;
        Ok(Assets {
            game,
            alert: font
                .texturize("TIME'S UP", &ColorRGBA(255, 0, 0, 255))?
                .at(align::bottom(360).center(640)),
            instructions: font
                .texturize("<PRESS ENTER>", &ColorRGBA(255, 255, 255, 255))?
                .at(align::top(360).center(640)),
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.game)?;
        {
            let dims = glm::uvec2(1080, 360);
            let pos = align::center(640).middle(360);
            //border
            renderer.set_draw_color(ColorRGBA(0, 0, 0, 255));
            renderer.fill_rects(&[pos.dims(dims)])?;
            //background
            renderer.set_draw_color(ColorRGBA(60, 0, 70, 255));
            renderer.fill_rects(&[pos.dims(glm::uvec2(dims.x - 12, dims.y - 12))])?;
        }
        renderer.show(&self.alert)?;
        renderer.show(&self.instructions)
    }
}
