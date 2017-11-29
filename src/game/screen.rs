use asset;
use data;
use game::{font, Helper};
use game::menu::{self, Menu};
use game::high_score::{self, HighScore};
use game::player_select::{self, PlayerSelect};

use moho::{self, input};
use moho::errors::*;
use moho::engine::{NextScene, World};
use moho::engine::step::fixed;
use moho::renderer::{Renderer, Scene, Texture};

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
    pub fn load<FM, AM>(
        font_manager: &mut FM,
        texture_manager: &mut AM,
        data: &data::Game,
        world: &Screen,
    ) -> Result<Self>
    where
        FM: font::Manager<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        match *world {
            Screen::Menu(ref m) => {
                menu::Assets::load(font_manager, texture_manager, data, m).map(Assets::Menu)
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
                ),
                _ => menu::Assets::load(
                    &mut helper.font_manager,
                    &mut helper.asset_manager,
                    &helper.data,
                    world,
                ),
            }.map(Assets::Menu),
            Screen::HighScore(_) => match self {
                Assets::HighScore(hs) => Ok(hs),
                _ => high_score::Assets::load(&mut helper.font_manager),
            }.map(Assets::HighScore),
            Screen::PlayerSelect(_) => match self {
                Assets::PlayerSelect(ps) => Ok(Assets::PlayerSelect(ps)),
                _ => {
                    player_select::Assets::load(&mut helper.font_manager).map(Assets::PlayerSelect)
                }
            },
        }
    }
}

impl<'t, R: Renderer<'t>> Scene<R> for Assets<R::Texture>
where
    R::Texture: Texture,
{
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Menu(ref m) => renderer.show(m),
            Assets::HighScore(ref hs) => renderer.show(hs),
            Assets::PlayerSelect(ref ps) => renderer.show(ps),
        }
    }
}
