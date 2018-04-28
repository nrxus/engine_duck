use game::text::{self, Text};
use {asset, Result};

use moho;
use moho::font::Font;
use moho::renderer::align;
use moho::texture::{Image, Texture};

use std::rc::Rc;
use std::time::Duration;

pub struct Hud {
    timer: Duration,
    pub score: u32,
}

impl Default for Hud {
    fn default() -> Self {
        Hud {
            timer: Duration::from_secs(100),
            score: 0,
        }
    }
}

impl Hud {
    pub fn update(self, scored: i32, elapsed: Duration) -> moho::State<Self, ()> {
        let score = if scored >= 0 {
            self.score + scored as u32
        } else {
            self.score.checked_sub(scored.abs() as u32).unwrap_or(0)
        };
        match self.timer.checked_sub(elapsed) {
            Some(timer) => moho::State::Running(Hud { timer, score }),
            None => moho::State::Quit(()),
        }
    }
}

impl text::Cached for Duration {
    type Value = u64;
    fn cached(&self) -> u64 {
        self.as_secs()
    }
}

impl text::Cached for u32 {
    type Value = u32;

    fn cached(&self) -> u32 {
        *self
    }
}

#[derive(Show)]
pub struct Assets<T, F> {
    timer: Image<Text<T, F, Duration>>,
    score: Image<Text<T, F, u32>>,
}

impl<T, F: Font<Texture = T>> Assets<T, F> {
    pub fn next(mut self, hud: &Hud) -> Result<Self> {
        self.timer.texture.update(hud.timer)?;
        Ok(self)
    }
}

impl<T: Texture, F: Font<Texture = T>> Assets<T, F> {
    pub fn load(
        world: &Hud,
        asset_manager: &mut impl asset::Manager<Texture = T, Font = F>,
    ) -> Result<Self> {
        let font = asset_manager.font(asset::Font::KenPixel, 32)?;
        Ok(Assets {
            timer: Text::load(world.timer, Rc::clone(&font), |v| format!("Time: {:03}", v))?
                .at(align::top(0).center(960)),
            score: Text::load(world.score, font, |v| format!("Score: {:05}", v))?
                .at(align::top(0).center(320)),
        })
    }
}
