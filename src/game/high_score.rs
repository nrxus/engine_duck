use asset::Image;
use game::font;

use moho::{self, input};
use moho::engine::World;
use moho::errors::*;
use moho::renderer::{align, ColorRGBA, Font, Renderer, Scene, Texture};
use moho::renderer::options::Position;
use sdl2::keyboard::Keycode;

use std::rc::Rc;
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
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);
        let center = align::center(640);

        let title = font_manager
            .load(font::Kind::KenPixel, 64)
            .and_then(|f| f.image("High Scores", &color, center.top(0)))?;

        let instructions = {
            let text = "<PRESS ENTER TO GO TO MAIN MENU>";
            let texture = font_manager
                .load(font::Kind::KenPixel, 32)
                .and_then(|f| f.texturize(text, &color).map_err(Into::into))
                .map(Rc::new)?;
            let dims = texture.dims();
            let dst = center.bottom(720 - dims.y as i32).dims(dims);
            Image { texture, dst }
        };

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
        for s in &self.scores {
            renderer.show(s)?;
        }
        Ok(())
    }
}

trait FontExt {
    type Texture: Texture;

    fn image(&self, text: &str, color: &ColorRGBA, pos: Position) -> Result<Image<Self::Texture>>;
}

impl<T> FontExt for T
where
    T: Font,
    T::Texture: Texture,
{
    type Texture = T::Texture;

    fn image(&self, text: &str, color: &ColorRGBA, pos: Position) -> Result<Image<Self::Texture>> {
        let texture = self.texturize(text, color).map(Rc::new)?;
        let dst = pos.dims(texture.dims());
        Ok(Image { texture, dst })
    }
}
