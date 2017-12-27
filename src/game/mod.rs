mod menu;
mod high_score;
mod player_select;
mod screen;
mod score_repository;
mod helper;

use self::screen::Screen;
use self::helper::Helper;
use asset;
use data;
use errors::*;

use moho::{self, input};
use moho::engine::{self, Engine, NextScene};
use moho::engine::step::fixed;
use moho::renderer::{self, ColorRGBA, Draw, Renderer};
use moho::texture::{self, Texture};

use std::error;
use std::time::Duration;

type State<W: engine::World> = moho::State<W, W::Quit>;

pub fn run<'t, 'f, E, C, FL, TL, T, Err: error::Error>(
    engine: &mut Engine<E, C, fixed::FixedUpdate>,
    texture_loader: &'t TL,
    font_loader: &'f FL,
) -> Result<()>
where
    T: Texture + Draw<C>,
    E: input::EventPump,
    C: renderer::Canvas,
    FL: moho::font::Loader<'f>,
    FL::Font: moho::font::Font<Texture = T>,
    TL: texture::Loader<'t, Texture = T, Error = Err>,
    moho::errors::Error: From<Err> + From<FL::Error>,
{
    let font_manager = moho::font::Manager::new(font_loader);
    let texture_manager = texture::Manager::new(texture_loader);
    let data = data::Game::load("media/game_data.yaml")?;
    let world = World {
        screen: Screen::new(data.animators()),
    };
    let mut helper = Helper {
        font_manager,
        texture_manager,
        data,
    };
    let scene = Assets::load(&world, &mut helper)?;
    engine
        .run::<Assets<TL::Texture>, _, _>(world, scene, helper)
        .map_err(Into::into)
}

pub struct World {
    screen: Screen,
}

impl engine::World for World {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> State<Self> {
        self.screen
            .update(input, elapsed)
            .map(|screen| World { screen })
    }
}

impl<AM: asset::Manager> NextScene<World, fixed::State, AM> for Assets<AM::Texture> {
    fn next(
        self,
        game: &World,
        step: &fixed::State,
        helper: &mut AM,
    ) -> moho::errors::Result<Self> {
        self.screen
            .next(&game.screen, step, helper)
            .map(|screen| Assets { screen })
    }
}

pub struct Assets<T> {
    screen: screen::Assets<T>,
}

impl<T: Texture> Assets<T> {
    fn load<AM>(world: &World, helper: &mut AM) -> moho::errors::Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        screen::Assets::load(&world.screen, helper).map(|screen| Assets { screen })
    }
}

impl<R: Renderer, T: Texture + Draw<R>> renderer::Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> moho::errors::Result<()> {
        renderer.show(&self.screen)?;
        //reset to the background color
        let color = ColorRGBA(60, 0, 70, 255);
        renderer.set_draw_color(color);
        Ok(())
    }
}
