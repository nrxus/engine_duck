use game::{self, font};

use moho::{self, input};
use moho::engine::World;
use moho::errors::*;
use moho::font::Font;
use moho::texture::{Image, Texture};
use moho::renderer::{align, ColorRGBA, Draw, Renderer, Show};
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct HighScore {}

impl World for HighScore {
    type Quit = ();

    fn update(self, input: &input::State, _: Duration) -> game::State<Self> {
        if input.did_press_key(Keycode::Return) {
            moho::State::Quit(())
        } else {
            moho::State::Running(self)
        }
    }
}

pub struct Assets<T> {
    title: Image<T>,
    instructions: Image<T>,
    scores: Vec<Image<T>>,
}

impl<T: Texture> Assets<T> {
    pub fn load<FM>(font_manager: &mut FM) -> Result<Self>
    where
        FM: font::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);
        let center = align::center(640);

        let title = {
            let text = "High Scores";
            let font = font_manager.load(font::Kind::KenPixel, 64)?;
            font.texturize(text, &color)?.at(center.top(0))
        };

        let instructions = {
            let text = "<PRESS ENTER TO GO TO MAIN MENU>";
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let height = font.measure(text)?.y as i32;
            font.texturize(text, &color)?
                .at(center.bottom(720 - height))
        };

        let scores = {
            let font = font_manager.load(font::Kind::Joystix, 32)?;
            let color = ColorRGBA(255, 255, 255, 255);
            let scores: Vec<_> = super::score_repository::get();

            let mut top = 150;
            let mut vec = Vec::with_capacity(scores.len());
            for s in scores {
                let score = format!("{:06}{:5}{:>6}", s.score, "", s.name);
                let image = font.texturize(&score, &color)?.at(center.top(top));
                top += image.dst.dims.y as i32;
                vec.push(image);
            }
            vec
        };

        Ok(Assets {
            title,
            instructions,
            scores,
        })
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.instructions)?;
        self.scores.iter().map(|s| renderer.show(s)).collect()
    }
}
