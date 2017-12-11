use glm;
use moho::errors::*;
use moho::{self, input};
use moho::engine::{NextScene, World};
use moho::font::Font;
use moho::texture::Texture;
use moho::renderer::{align, options, Draw, Renderer, Show};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

pub use self::button::Kind as Quit;

pub struct Gui {
    selected: button::Kind,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            selected: button::Kind::NewGame,
        }
    }
}

impl World for Gui {
    type Quit = Quit;

    fn update(mut self, input: &input::State, _: Duration) -> moho::State<Self, Quit> {
        if input.did_press_key(Keycode::Down) ^ input.did_press_key(Keycode::Up) {
            self.selected = match self.selected {
                button::Kind::NewGame => button::Kind::HighScore,
                button::Kind::HighScore => button::Kind::NewGame,
            }
        }
        if input.did_press_key(Keycode::Return) {
            moho::State::Quit(self.selected)
        } else {
            moho::State::Running(self)
        }
    }
}

impl<T: Texture> NextScene<Gui, (), ()> for Assets<T> {
    fn next(mut self, gui: &Gui, _: &(), _: &mut ()) -> Result<Self> {
        self.selected = gui.selected;
        Ok(self)
    }
}

pub struct Assets<T> {
    selected: button::Kind,
    new_game: button::Assets<T>,
    high_score: button::Assets<T>,
    picker: Rc<T>,
}

impl<T> Assets<T> {
    pub fn load<F>(font: &F, picker: Rc<T>, gui: &Gui) -> Result<Self>
    where
        F: Font<Texture = T>,
    {
        let new_game = {
            let center = glm::ivec2(640, 325);
            let text = "New Game";
            button::Assets::load(text, font, center)
        }?;
        let high_score = {
            let center = glm::ivec2(640, 500);
            let text = "High Scores";
            button::Assets::load(text, font, center)
        }?;
        Ok(Assets {
            new_game,
            high_score,
            picker,
            selected: gui.selected,
        })
    }
}

impl<R: Renderer, T: Draw<R> + Texture> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        let (selected, unselected) = match self.selected {
            button::Kind::HighScore => (&self.high_score, &self.new_game),
            button::Kind::NewGame => (&self.new_game, &self.high_score),
        };
        //selected + picker
        {
            let rect = selected.dst.rect();
            let dst = align::right(rect.x - 10)
                .middle(rect.y + rect.w / 2)
                .dims(self.picker.dims());
            renderer.draw(&*self.picker, options::at(dst))?;
            renderer.draw(&*selected.selected, options::at(selected.dst))
        }?;
        //unselected
        {
            renderer.draw(&*unselected.idle, options::at(unselected.dst))
        }
    }
}

mod button {
    use glm;
    use moho::errors::*;
    use moho::font::Font;
    use moho::renderer::{align, options, ColorRGBA};

    use std::rc::Rc;

    #[derive(Clone, Copy)]
    pub enum Kind {
        NewGame,
        HighScore,
    }

    pub struct Assets<T> {
        pub idle: Rc<T>,
        pub selected: Rc<T>,
        pub dst: options::Destination,
    }

    impl<T> Assets<T> {
        pub fn load<F>(text: &str, font: &F, center: glm::IVec2) -> Result<Self>
        where
            F: Font<Texture = T>,
        {
            let dims = font.measure(text)?;
            let idle = font.texturize(text, &ColorRGBA(255, 255, 255, 255))
                .map(Rc::new)?;
            let selected = font.texturize(text, &ColorRGBA(255, 255, 0, 255))
                .map(Rc::new)?;
            let dst = align::middle(center.y).center(center.x).dims(dims);
            Ok(Assets {
                idle,
                selected,
                dst,
            })
        }
    }
}
