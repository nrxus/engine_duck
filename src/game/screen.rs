use asset;
use data::Animators;
use game;
use game::game_play::{self, GamePlay};
use game::menu::{self, Menu};
use game::high_score::{self, HighScore};
use game::player_select::{self, PlayerSelect};

use moho::{self, input, Never};
use moho::errors::*;
use moho::engine::{NextScene, World};
use moho::engine::step::fixed;
use moho::texture::Texture;
use moho::renderer::{Draw, Renderer, Show};

use std::time::Duration;

pub struct Screen {
    animators: Animators,
    current: Kind,
}

impl Screen {
    pub fn new(animators: Animators) -> Self {
        Screen {
            animators,
            current: Kind::Menu(Menu::default()),
        }
    }
}

pub enum Kind {
    Menu(Menu),
    HighScore(HighScore),
    PlayerSelect(PlayerSelect),
    GamePlay(GamePlay),
}

pub enum Assets<T> {
    Menu(menu::Assets<T>),
    HighScore(high_score::Assets<T>),
    PlayerSelect(player_select::Assets<T>),
    GamePlay(game_play::Assets),
}

impl World for Screen {
    type Quit = Never;

    fn update(self, input: &input::State, elapsed: Duration) -> game::State<Self> {
        let animators = self.animators;
        let current = match self.current {
            Kind::Menu(m) => m.update(input, elapsed)
                .map(Kind::Menu)
                .catch_quit(|b| match b {
                    menu::Quit::NewGame => Kind::PlayerSelect(PlayerSelect::new(&animators)),
                    menu::Quit::HighScore => Kind::HighScore(HighScore {}),
                }),
            Kind::HighScore(hs) => hs.update(input, elapsed)
                .map(Kind::HighScore)
                .catch_quit(|_| Kind::Menu(Menu::default())),
            Kind::PlayerSelect(ps) => ps.update(input, elapsed)
                .map(Kind::PlayerSelect)
                .catch_quit(|_| Kind::GamePlay(GamePlay::new())),
            Kind::GamePlay(gp) => gp.update(input, elapsed)
                .map(Kind::GamePlay)
                .catch_quit(|_| Kind::Menu(Menu::default())),
        };
        moho::State::Running(Screen { current, animators })
    }
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(world: &Screen, asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        match world.current {
            Kind::Menu(ref m) => menu::Assets::load(asset_manager, m).map(Assets::Menu),
            Kind::HighScore(_) => high_score::Assets::load(asset_manager).map(Assets::HighScore),
            Kind::PlayerSelect(_) => {
                player_select::Assets::load(asset_manager).map(Assets::PlayerSelect)
            }
            Kind::GamePlay(_) => game_play::Assets::load().map(Assets::GamePlay),
        }
    }
}

impl<AM: asset::Manager> NextScene<Screen, fixed::State, AM> for Assets<AM::Texture> {
    fn next(self, screen: &Screen, _: &fixed::State, helper: &mut AM) -> Result<Self> {
        match screen.current {
            Kind::Menu(ref world) => match self {
                Assets::Menu(m) => m.next(world, &(), &mut ()).map(Assets::Menu),
                _ => Assets::load(screen, helper),
            },
            Kind::HighScore(_) => match self {
                hs @ Assets::HighScore(_) => Ok(hs),
                _ => Assets::load(screen, helper),
            },
            Kind::PlayerSelect(ref world) => match self {
                Assets::PlayerSelect(ps) => ps.next(world, &(), &mut ()).map(Assets::PlayerSelect),
                _ => Assets::load(screen, helper),
            },
            Kind::GamePlay(ref world) => match self {
                Assets::GamePlay(ps) => ps.next(world, &(), &mut ()).map(Assets::GamePlay),
                _ => Assets::load(screen, helper),
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
            Assets::GamePlay(ref gp) => renderer.show(gp),
        }
    }
}
