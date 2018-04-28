use game::game_play::{self, GamePlay};
use game::high_score::{self, HighScore};
use game::menu::{self, Menu};
use game::player_select::{self, PlayerSelect};
use {asset, Result};

use moho::engine::step::fixed;
use moho::font::Font;
use moho::texture::Texture;
use moho::{self, input};

use std::time::Duration;

pub enum Quit {
    Menu(menu::Quit),
    HighScore,
    PlayerSelect(player_select::PlayerKind),
    GamePlay,
}

impl Screen {
    pub fn new() -> Self {
        Screen::Menu(Menu::default())
    }

    pub fn update(self, input: &input::State, elapsed: Duration) -> moho::State<Self, Quit> {
        match self {
            Screen::Menu(m) => m.update(input).map(Screen::Menu).map_quit(Quit::Menu),
            Screen::HighScore(hs) => hs
                .update(input)
                .map(Screen::HighScore)
                .map_quit(|_| Quit::HighScore),
            Screen::PlayerSelect(ps) => ps
                .update(input, elapsed)
                .map(Screen::PlayerSelect)
                .map_quit(Quit::PlayerSelect),
            Screen::GamePlay(gp) => gp
                .update(input, elapsed)
                .map(Screen::GamePlay)
                .map_quit(|_| Quit::GamePlay),
        }
    }
}

pub enum Screen {
    Menu(Menu),
    HighScore(HighScore),
    PlayerSelect(PlayerSelect),
    GamePlay(GamePlay),
}

#[derive(Show)]
pub enum Assets<T, F> {
    Menu(menu::Assets<T>),
    HighScore(high_score::Assets<T>),
    PlayerSelect(player_select::Assets<T>),
    GamePlay(game_play::Assets<T, F>),
}

impl<T: Texture + Clone, F: Font<Texture = T>> Assets<T, F> {
    pub fn load(
        screen: &Screen,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        match *screen {
            Screen::Menu(ref m) => menu::Assets::load(m, asset_manager).map(Assets::Menu),
            Screen::HighScore(_) => high_score::Assets::load(asset_manager).map(Assets::HighScore),
            Screen::PlayerSelect(_) => {
                player_select::Assets::load(asset_manager).map(Assets::PlayerSelect)
            }
            Screen::GamePlay(ref gp) => {
                game_play::Assets::load(gp, asset_manager).map(Assets::GamePlay)
            }
        }
    }
}

impl<T: Texture + Clone, F: Font<Texture = T>> Assets<T, F> {
    pub fn next(
        self,
        screen: &Screen,
        step: &fixed::State,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        match *screen {
            Screen::Menu(ref world) => match self {
                Assets::Menu(m) => Ok(m.next(world)),
                _ => menu::Assets::load(world, asset_manager),
            }
            .map(Assets::Menu),
            Screen::HighScore(_) => match self {
                Assets::HighScore(hs) => Ok(hs),
                _ => high_score::Assets::load(asset_manager),
            }
            .map(Assets::HighScore),
            Screen::PlayerSelect(ref world) => match self {
                Assets::PlayerSelect(ps) => Ok(ps.next(world)),
                _ => player_select::Assets::load(asset_manager),
            }
            .map(Assets::PlayerSelect),
            Screen::GamePlay(ref world) => match self {
                Assets::GamePlay(ps) => ps.next(world, step, asset_manager),
                _ => game_play::Assets::load(world, asset_manager),
            }
            .map(Assets::GamePlay),
        }
    }
}
