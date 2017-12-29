use asset;
use data::Animators;
use game::game_play::{self, GamePlay};
use game::high_score::{self, HighScore};
use game::menu::{self, Menu};
use game::player_select::{self, PlayerSelect};
use game::timeup::{self, TimeUp};

use moho::input;
use moho::errors::*;
use moho::engine::step::fixed;
use moho::font::Font;
use moho::renderer::{Draw, Renderer, Show};
use moho::texture::Texture;

use std::time::Duration;

impl Screen {
    pub fn new() -> Self {
        Screen::Menu(Menu::default())
    }

    pub fn update(self, input: &input::State, elapsed: Duration, animators: &Animators) -> Self {
        match self {
            Screen::Menu(m) => m.update(input).map(Screen::Menu).catch_quit(|b| match b {
                menu::Quit::NewGame => Screen::PlayerSelect(PlayerSelect::new(animators)),
                menu::Quit::HighScore => Screen::HighScore(HighScore {}),
            }),
            Screen::HighScore(hs) => hs.update(input)
                .map(Screen::HighScore)
                .catch_quit(|_| Screen::Menu(Menu::default())),
            Screen::PlayerSelect(ps) => ps.update(input, elapsed)
                .map(Screen::PlayerSelect)
                .catch_quit(|_| Screen::GamePlay(GamePlay::new())),
            Screen::GamePlay(gp) => gp.update(input, elapsed)
                .map(Screen::GamePlay)
                .catch_quit(|_| Screen::TimeUp(TimeUp {})),
            Screen::TimeUp(tu) => tu.update(input)
                .map(Screen::TimeUp)
                .catch_quit(|_| Screen::Menu(Menu::default())),
        }
    }
}

pub enum Screen {
    Menu(Menu),
    HighScore(HighScore),
    PlayerSelect(PlayerSelect),
    GamePlay(GamePlay),
    TimeUp(TimeUp),
}

pub enum Assets<T, F> {
    Menu(menu::Assets<T>),
    HighScore(high_score::Assets<T>),
    PlayerSelect(player_select::Assets<T>),
    GamePlay(game_play::Assets<T, F>),
    TimeUp(timeup::Assets<T, F>),
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load<AM>(screen: &Screen, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        match *screen {
            Screen::Menu(ref m) => menu::Assets::load(m, asset_manager).map(Assets::Menu),
            Screen::HighScore(_) => high_score::Assets::load(asset_manager).map(Assets::HighScore),
            Screen::PlayerSelect(_) => {
                player_select::Assets::load(asset_manager).map(Assets::PlayerSelect)
            }
            Screen::GamePlay(ref gp) => {
                game_play::Assets::load(gp, asset_manager).map(Assets::GamePlay)
            }
            _ => unreachable!("not an initial screen"),
        }
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn next<AM>(
        self,
        screen: &Screen,
        step: &fixed::State,
        asset_manager: &mut AM,
    ) -> Result<Self>
    where
        AM: asset::Manager<Texture = T, Font = F>,
    {
        match *screen {
            Screen::Menu(ref world) => match self {
                Assets::Menu(m) => Ok(m.next(world)),
                _ => menu::Assets::load(world, asset_manager),
            }.map(Assets::Menu),
            Screen::HighScore(_) => match self {
                Assets::HighScore(hs) => Ok(hs),
                _ => high_score::Assets::load(asset_manager),
            }.map(Assets::HighScore),
            Screen::PlayerSelect(ref world) => match self {
                Assets::PlayerSelect(ps) => Ok(ps.next(world)),
                _ => player_select::Assets::load(asset_manager),
            }.map(Assets::PlayerSelect),
            Screen::GamePlay(ref world) => match self {
                Assets::GamePlay(ps) => ps.next(world, step),
                _ => game_play::Assets::load(world, asset_manager),
            }.map(Assets::GamePlay),
            Screen::TimeUp(_) => match self {
                Assets::TimeUp(tu) => Ok(tu),
                Assets::GamePlay(gp) => timeup::Assets::load(asset_manager, gp),
                _ => unreachable!("can only be loaded from a previous GamePlay"),
            }.map(Assets::TimeUp),
        }
    }
}

impl<R: Renderer, T: Draw<R> + Texture, F> Show<R> for Assets<T, F> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Assets::Menu(ref m) => renderer.show(m),
            Assets::HighScore(ref hs) => renderer.show(hs),
            Assets::PlayerSelect(ref ps) => renderer.show(ps),
            Assets::GamePlay(ref gp) => renderer.show(gp),
            Assets::TimeUp(ref tu) => renderer.show(tu),
        }
    }
}
