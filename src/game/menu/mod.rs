mod gui;

pub use self::gui::Quit;
use self::gui::Gui;
use asset::{self, Image};
use data;
use game::font;

use moho::{self, input};
use moho::errors::*;
use moho::engine::{NextScene, World};
use moho::engine::step::fixed;
use moho::renderer::{align, ColorRGBA, Font, Renderer, Scene, Texture};

use std::rc::Rc;
use std::time::Duration;

#[derive(Default)]
pub struct Menu {
    gui: Gui,
}

impl World for Menu {
    type Quit = Quit;

    fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, Self::Quit> {
        self.gui.update(input, elapsed).map(|gui| Menu { gui })
    }
}

impl<T: Texture> NextScene<Menu, fixed::State, ()> for Assets<T> {
    fn next(mut self, snapshot: ::RefSnapshot<Menu>, _: &mut ()) -> Result<Self> {
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
    pub fn load<FM, AM>(
        font_manager: &mut FM,
        texture_manager: &mut AM,
        data: &data::Game,
        menu: &Menu,
    ) -> Result<Self>
    where
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
        AM: asset::Loader<Texture = T>,
    {
        let husky = {
            let pos = align::right(640 - 32 - 30).middle(125);
            let player = &data.husky;
            let texture = texture_manager.texture(&player.idle_texture)?;
            let dst = player.out_size.dst(pos).scale(2);
            Image { texture, dst }
        };
        let duck = {
            let pos = align::left(640 + 32 + 30).middle(125);
            let player = &data.duck;
            let texture = texture_manager.texture(&player.idle_texture)?;
            let dst = player.out_size.dst(pos).scale(2);
            Image { texture, dst }
        };
        let heart = {
            let pos = align::center(640).middle(125);
            let data = &data.heart;
            let texture = texture_manager.texture(&data.texture)?;
            let dst = data.out_size.dst(pos).scale(2);
            Image { texture, dst }
        };
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
            let picker = texture_manager.texture(&data.heart.texture)?;
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
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.husky)?;
        renderer.show(&self.duck)?;
        renderer.show(&self.heart)?;
        renderer.show(&self.instructions)?;
        renderer.show(&self.gui)
    }
}
