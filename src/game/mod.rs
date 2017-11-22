mod menu;
mod font;

use self::menu::Menu;
use data;
use errors::*;

use moho::{self, input};
use moho::engine::{self, Engine, NextScene};
use moho::engine::step::fixed;
use moho::renderer::{self, ColorRGBA, Font, FontLoader, Renderer, Texture, TextureLoader,
                     TextureManager};
use moho::renderer::font as moho_font;

use std::time::Duration;

pub fn run<'t, 'f, E, C, FL, TL, T: Texture>(
    engine: &mut Engine<E, C, fixed::FixedUpdate>,
    texture_loader: &'t TL,
    font_loader: &'f FL,
) -> Result<()>
where
    E: input::EventPump,
    C: renderer::Canvas<'t, Texture = T>,
    C::Texture: Texture,
    FL: FontLoader<'f>,
    FL::Font: Font<Texture = T>,
    TL: TextureLoader<'t, Texture = T>,
{
    let mut font_manager = moho_font::Manager::new(font_loader);
    let mut texture_manager = TextureManager::new(texture_loader);
    let data = data::Game::load("media/game_data.yaml")?;
    let world = World::default();
    let scene = Assets::load(&mut font_manager, &mut texture_manager, &data, &world)?;
    let helper = Helper {};
    engine
        .run::<Assets<C::Texture>, _, _>(world, scene, helper)
        .map_err(Into::into)
}

#[derive(Default)]
pub struct World {
    menu: Menu,
}

impl engine::World for World {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        self.menu.update(input, elapsed).map(|menu| World { menu })
    }
}

impl<T: Texture> NextScene<World, fixed::State, Helper> for Assets<T> {
    fn next(self, snapshot: ::RefSnapshot<World>, _: &mut Helper) -> moho::errors::Result<Self> {
        self.menu
            .next(snapshot.split(|w| &w.menu), &mut ())
            .map(|menu| Assets { menu })
    }
}

pub struct Helper {}

pub struct Assets<T> {
    menu: menu::Assets<T>,
}

impl<T: Texture> Assets<T> {
    fn load<'t, FM, TL>(
        font_manager: &mut FM,
        texture_manager: &mut TextureManager<'t, TL>,
        data: &data::Game,
        world: &World,
    ) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
        FM: font::Manager,
        FM::Font: Font<Texture = T>,
    {
        let menu = menu::Assets::load(font_manager, texture_manager, data, &world.menu)?;
        Ok(Assets { menu })
    }
}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for Assets<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.show(&self.menu)?;
        //reset to the background color
        let color = ColorRGBA(60, 0, 70, 255);
        renderer.set_draw_color(color);
        Ok(())
    }
}
