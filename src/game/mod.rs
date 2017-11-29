mod menu;
mod high_score;
mod player_select;
mod font;
mod screen;
mod score_repository;

use self::screen::Screen;
use self::menu::Menu;
use asset;
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
    let font_manager = moho_font::Manager::new(font_loader);
    let asset_manager = TextureManager::new(texture_loader);
    let data = data::Game::load("media/game_data.yaml")?;
    let world = World {
        screen: Screen::Menu(Menu::default()),
    };
    let mut helper = Helper {
        font_manager,
        asset_manager,
        data,
    };
    let scene = Assets::load(
        &mut helper.font_manager,
        &mut helper.asset_manager,
        &helper.data,
        &world,
    )?;
    engine
        .run::<Assets<C::Texture>, _, _>(world, scene, helper)
        .map_err(Into::into)
}

pub struct World {
    screen: Screen,
}

impl engine::World for World {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        self.screen
            .update(input, elapsed)
            .map(|screen| World { screen })
    }
}

impl<FM, AM> NextScene<World, fixed::State, Helper<FM, AM>> for Assets<AM::Texture>
where
    AM: asset::Loader,
    AM::Texture: Texture,
    FM: font::Manager<Texture = AM::Texture>,
{
    fn next(
        self,
        snapshot: ::RefSnapshot<World>,
        helper: &mut Helper<FM, AM>,
    ) -> moho::errors::Result<Self> {
        self.screen
            .next(snapshot.split(|w| &w.screen), helper)
            .map(|screen| Assets { screen })
    }
}

pub struct Helper<FM, AM> {
    font_manager: FM,
    asset_manager: AM,
    data: data::Game,
}

pub struct Assets<T> {
    screen: screen::Assets<T>,
}

impl<T: Texture> Assets<T> {
    fn load<'t, FM, AM>(
        font_manager: &mut FM,
        asset_manager: &mut AM,
        data: &data::Game,
        world: &World,
    ) -> moho::errors::Result<Self>
    where
        AM: asset::Loader<Texture = T>,
        FM: font::Manager<Texture = T>,
    {
        screen::Assets::load(font_manager, asset_manager, data, &world.screen)
            .map(|screen| Assets { screen })
    }
}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for Assets<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.show(&self.screen)?;
        //reset to the background color
        let color = ColorRGBA(60, 0, 70, 255);
        renderer.set_draw_color(color);
        Ok(())
    }
}
