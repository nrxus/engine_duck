use asset::Image;
use game::font::{self, FontExt};

use moho::{self, input};
use moho::engine::World;
use moho::errors::*;
use moho::renderer::{align, ColorRGBA, Font, Renderer, Scene, Texture};
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct HighScore {}

impl World for HighScore {
    type Quit = ();

    fn update(self, input: &input::State, _: Duration) -> moho::State<Self, ()> {
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

        let title = font_manager
            .load(font::Kind::KenPixel, 64)
            .and_then(|f| f.image("High Scores", &color, center.top(0)))?;

        let instructions = {
            let text = "<PRESS ENTER TO GO TO MAIN MENU>";
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let height = font.measure(text)?.y as i32;
            font.image(text, &color, center.bottom(720 - height))
        }?;

        let scores = {
            let font = font_manager.load(font::Kind::Joystix, 32)?;
            let color = ColorRGBA(255, 255, 255, 255);
            let scores: Vec<_> = super::score_repository::get();

            let mut top = 150;
            let mut vec = Vec::with_capacity(scores.len());
            for s in scores {
                let score = format!("{:06}{:5}{:>6}", s.score, "", s.name);
                let image = font.image(&score, &color, center.top(top))?;
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

impl<'t, R: Renderer<'t>> Scene<R> for Assets<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.title)?;
        renderer.show(&self.instructions)?;
        self.scores.iter().map(|s| renderer.show(s)).collect()
    }
}
