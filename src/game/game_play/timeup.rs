use asset;
use super::running;

use glm;
use moho::{self, input};
use moho::errors::*;
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};
use moho::font::Font;
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

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
    pub fn load<AM>(asset_manager: &mut AM, game: running::Assets<T, F>) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let font = asset_manager.font(asset::Font::KenPixel, 48)?;
        Ok(Assets {
            game,
            alert: font.texturize("TIME'S UP", &ColorRGBA(255, 0, 0, 255))?
                .at(align::bottom(360).center(640)),
            instructions: font.texturize("<PRESS ENTER>", &ColorRGBA(255, 255, 255, 255))?
                .at(align::top(360).center(640)),
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.game)?;
        {
            let (x, y) = (1080, 360);
            let view = glm::ivec4(640 - x / 2, 360 - y / 2, x, y);
            //border
            renderer.set_draw_color(ColorRGBA(0, 0, 0, 255));
            renderer.fill_rects(&[Rect::new(view.x, view.y, view.z as u32, view.w as u32)])?;
            //background
            renderer.set_draw_color(ColorRGBA(60, 0, 70, 255));
            renderer.fill_rects(&[
                Rect::new(
                    view.x + 6,
                    view.y + 6,
                    view.z as u32 - 12,
                    view.w as u32 - 12,
                ),
            ])?;
        }
        renderer.show(&self.alert)?;
        renderer.show(&self.instructions)
    }
}
