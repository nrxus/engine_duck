use asset;
use game::{font, Helper};
use game::menu::{self, Menu};
use game::high_score::{self, HighScore};
use game::player_select::{self, PlayerSelect};

use moho::{self, input};
use moho::errors::*;
use moho::engine::{NextScene, World};
use moho::engine::step::fixed;
use moho::texture::Texture;
use moho::renderer::{Draw, Renderer, Show};

use std::time::Duration;

pub enum Screen {
    Menu(Menu),
    HighScore(HighScore),
    PlayerSelect(PlayerSelect),
}

pub enum Assets<T> {
    Menu(menu::Assets<T>),
    HighScore(high_score::Assets<T>),
    PlayerSelect(player_select::Assets<T>),
}

impl World for Screen {
    type Quit = ();

    fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, ()> {
        match self {
            Screen::Menu(m) => m.update(input, elapsed).map(Screen::Menu).flat_map_quit(
                |b| match b {
                    menu::Quit::NewGame => {
                        moho::State::Running(Screen::PlayerSelect(PlayerSelect {}))
                    }
                    menu::Quit::HighScore => moho::State::Running(Screen::HighScore(HighScore {})),
                },
            ),
            Screen::HighScore(hs) => hs.update(input, elapsed)
                .map(Screen::HighScore)
                .flat_map_quit(|_| moho::State::Running(Screen::Menu(Menu::default()))),
            Screen::PlayerSelect(ps) => moho::State::Running(Screen::PlayerSelect(ps)),
        }
    }
}

impl<T: Texture> Assets<T> {
    pub fn load<FM, AM>(world: &Screen, helper: &mut Helper<FM, AM>) -> Result<Self>
    where
        FM: font::Manager<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let font_manager = &mut helper.font_manager;
        let asset_manager = &mut helper.asset_manager;
        let data = &helper.data;

        match *world {
            Screen::Menu(ref m) => {
                menu::Assets::load(font_manager, asset_manager, data, m).map(Assets::Menu)
            }
            Screen::HighScore(_) => high_score::Assets::load(font_manager).map(Assets::HighScore),
            Screen::PlayerSelect(_) => {
                player_select::Assets::load(font_manager).map(Assets::PlayerSelect)
            }
        }
    }
}

impl<FM, AM> NextScene<Screen, fixed::State, Helper<FM, AM>> for Assets<AM::Texture>
where
    AM: asset::Manager,
    FM: font::Manager<Texture = AM::Texture>,
{
    fn next(self, snapshot: ::RefSnapshot<Screen>, helper: &mut Helper<FM, AM>) -> Result<Self> {
        match *snapshot.world {
            Screen::Menu(ref world) => match self {
                Assets::Menu(m) => m.next(
                    ::RefSnapshot {
                        world,
                        step_state: snapshot.step_state,
                    },
                    &mut (),
                ).map(Assets::Menu),
                _ => Assets::load(snapshot.world, helper),
            },
            Screen::HighScore(_) => match self {
                hs @ Assets::HighScore(_) => Ok(hs),
                _ => Assets::load(snapshot.world, helper),
            },
            Screen::PlayerSelect(_) => match self {
                ps @ Assets::PlayerSelect(_) => Ok(ps),
                _ => Assets::load(snapshot.world, helper),
            },
        }
    }
}

impl<R: Renderer, T: Draw<R> + Texture> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Menu(ref m) => renderer.show(m),
            Assets::HighScore(ref hs) => renderer.show(hs),
            Assets::PlayerSelect(ref ps) => renderer.show(ps),
        }
    }
}
