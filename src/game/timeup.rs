use asset;
use game::{self, game_play};

use moho::{self, input};
use moho::engine::World;
use moho::errors::*;
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};
use moho::font::Font;
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct TimeUp {}

impl World for TimeUp {
    type Quit = ();

    fn update(self, input: &input::State, _: Duration) -> game::State<Self> {
        if input.did_press_key(Keycode::Return) {
            moho::State::Quit(())
        } else {
            moho::State::Running(self)
        }
    }
}

pub struct Assets<T, F> {
    game: game_play::Assets<T, F>,
    alert: Image<T>,
    instructions: Image<T>,
}

impl<T: Texture, F> Assets<T, F> {
    pub fn load<AM>(asset_manager: &mut AM, game: game_play::Assets<T, F>) -> Result<Self>
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
        renderer.show(&self.alert)?;
        renderer.show(&self.instructions)
    }
}
