mod game_play;
mod helper;
mod high_score;
mod hud;
mod menu;
mod player;
mod player_select;
mod score_repository;
mod screen;
mod text;

use self::screen::Screen;
use self::helper::Helper;
use {asset, data, Result};

use moho::{self, input};
use moho::engine::{self, Engine, NextScene};
use moho::engine::step::fixed;
use moho::font::Font;
use moho::renderer::{self, ColorRGBA, Draw, Renderer};
use moho::texture::{self, Texture};

use std::time::Duration;
use std::rc::Rc;

type State<W> = moho::State<W, <W as engine::World>::Quit>;

pub fn run<'t, 'f, E, C, FL, TL, T>(
    engine: &mut Engine<E, C, fixed::FixedUpdate>,
    texture_loader: &'t TL,
    font_loader: &'f FL,
) -> Result<()>
where
    T: Texture + Draw<C>,
    E: input::EventPump,
    C: renderer::Canvas,
    FL: moho::font::Loader<'f>,
    FL::Font: moho::font::Font<Texture = Rc<T>>,
    TL: texture::Loader<'t, Texture = T>,
{
    let font_manager = moho::font::Manager::new(font_loader);
    let texture_manager = texture::Manager::new(texture_loader);
    let data = data::Game::load("media/game_data.yaml")?;
    let world = World {
        animators: data.animators(),
        screen: Screen::new(),
    };
    let mut helper = Helper {
        font_manager,
        texture_manager,
        data,
    };
    let scene = Assets::load(&world, &mut helper)?;
    engine.run(world, scene, helper).map_err(Into::into)
}

pub struct World {
    screen: Screen,
    animators: data::Animators,
}

impl engine::World for World {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> State<Self> {
        let animators = self.animators;
        let screen = self.screen.update(input, elapsed).catch_quit(|q| match q {
            screen::Quit::Menu(m) => match m {
                menu::Quit::NewGame => {
                    Screen::PlayerSelect(player_select::PlayerSelect::new(&animators))
                }
                menu::Quit::HighScore => Screen::HighScore(high_score::HighScore {}),
            },
            screen::Quit::HighScore | screen::Quit::GamePlay => Screen::Menu(menu::Menu::default()),
            screen::Quit::PlayerSelect(k) => {
                Screen::GamePlay(game_play::GamePlay::new(k, &animators))
            }
        });
        moho::State::Running(World { screen, animators })
    }
}

impl<AM: asset::Manager> NextScene<World, fixed::State, AM> for Assets<AM::Texture, AM::Font>
where
    AM::Texture: Clone,
{
    fn next(self, game: &World, step: &fixed::State, helper: &mut AM) -> Result<Self> {
        self.screen
            .next(&game.screen, step, helper)
            .map(|screen| Assets { screen })
    }
}

pub struct Assets<T, F> {
    screen: screen::Assets<T, F>,
}

impl<T: Texture + Clone, F: Font<Texture = T>> Assets<T, F> {
    fn load<AM>(world: &World, helper: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        screen::Assets::load(&world.screen, helper).map(|screen| Assets { screen })
    }
}

impl<R: Renderer, T: Texture + Draw<R>, F> renderer::Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.screen)?;
        //reset to the background color
        let color = ColorRGBA(60, 0, 70, 255);
        renderer.set_draw_color(color);
        Ok(())
    }
}
