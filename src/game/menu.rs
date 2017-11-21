use self::gui::Gui;
use asset::{AssetLoader, Image};
use data;
use errors::*;
use game::font;

use moho::{self, input};
use moho::engine::{self, NextScene, World};
use moho::engine::step::fixed;
use moho::renderer::{align, ColorRGBA, Font, Renderer, Scene, Texture, TextureLoader,
                     TextureManager};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
use std::time::Duration;

#[derive(Default)]
pub struct Menu {
    gui: Gui,
}

impl World for Menu {
    fn update(self, input: &input::State, elapsed: Duration) -> engine::State<Self> {
        self.gui.update(input, elapsed).map(|gui| Menu { gui })
    }
}

impl<T: Texture> NextScene<Menu, fixed::State, ()> for Assets<T> {
    fn next(mut self, snapshot: ::RefSnapshot<Menu>, _: &mut ()) -> moho::errors::Result<Self> {
        let snapshot = snapshot.split(|s| &s.gui);
        self.gui = self.gui.next(snapshot, &mut ())?;
        Ok(self)
    }
}

pub struct Assets<T> {
    husky: Image<T>,
    duck: Image<T>,
    heart: Image<T>,
    instructions: Image<T>,
    gui: gui::Assets<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<'t, FM, TL>(
        font_manager: &mut FM,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
        menu: &Menu,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let husky = {
            let dst = align::right(640 - 32 - 30).middle(125);
            texture_manager.load_player_image(&data.husky, dst, 2)
        }?;
        let duck = {
            let dst = align::left(640 + 32 + 30).middle(125);
            texture_manager.load_player_image(&data.duck, dst, 2)
        }?;
        let heart = {
            let dst = align::center(640).middle(125);
            texture_manager.load_image(&data.heart, dst, 2)
        }?;
        let instructions = {
            let font = font_manager.load(font::Kind::KenPixel, 32)?;
            let color = ColorRGBA(255, 255, 0, 255);
            let text = "<Use Arrow Keys to select option; then press Enter>";
            let texture = font.texturize(text, &color).map(Rc::new)?;
            let dims = texture.dims();
            let dst = align::bottom(720 - dims.y as i32).center(640).dims(dims);
            Image { texture, dst }
        };
        let gui = {
            let picker = texture_manager.load_texture(&data.heart.texture)?;
            let font = font_manager.load(font::Kind::KenPixel, 64)?;
            gui::Assets::load(&*font, picker, &menu.gui)
        }?;

        Ok(Assets {
            husky,
            duck,
            heart,
            instructions,
            gui,
        })
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Assets<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.show(&self.heart)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}

mod gui {
    use errors::*;
    use super::button;

    use glm;
    use moho::{self, input};
    use moho::engine::step::fixed;
    use moho::engine::{self, NextScene, World};
    use moho::renderer::{align, options, Font, Renderer, Scene, Texture};
    use sdl2::keyboard::Keycode;

    use std::rc::Rc;
    use std::time::Duration;

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
        fn update(mut self, input: &input::State, _: Duration) -> engine::State<Self> {
            if input.did_press_key(Keycode::Down) ^ input.did_press_key(Keycode::Up) {
                self.selected = match self.selected {
                    button::Kind::NewGame => button::Kind::HighScore,
                    button::Kind::HighScore => button::Kind::NewGame,
                }
            }
            engine::State::Running(self)
        }
    }

    impl<T: Texture> NextScene<Gui, fixed::State, ()> for Assets<T> {
        fn next(mut self, snapshot: ::RefSnapshot<Gui>, _: &mut ()) -> moho::errors::Result<Self> {
            self.selected = snapshot.world.selected;
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

    impl<'t, R: Renderer<'t>> Scene<R> for Assets<R::Texture>
    where
        R::Texture: Texture,
    {
        fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
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
                renderer.copy(&*self.picker, options::at(dst))?;
                renderer.copy(&*selected.selected, options::at(selected.dst))
            }?;
            //unselected
            {
                renderer.copy(&*unselected.idle, options::at(unselected.dst))
            }
        }
    }
}

mod button {
    use errors::*;

    use glm;
    use moho::renderer::{align, options, ColorRGBA, Font};

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
