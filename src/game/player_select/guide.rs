use asset::{self, Sprite};
use data::Animators;
use Result;

use moho::animation::animator::Animator;
use moho::font::Font;
use moho::renderer::{align, ColorRGBA};
use moho::texture::{Image, Texture};

use std::time::Duration;

pub struct Guide {
    gem: Animator,
    coin: Animator,
    cat: Animator,
}

impl Guide {
    pub fn new(animators: &Animators) -> Self {
        Guide {
            gem: animators.gem.start(),
            coin: animators.coin.start(),
            cat: animators.cat_idle.start(),
        }
    }

    pub fn update(mut self, elapsed: Duration) -> Self {
        self.coin.animate(elapsed);
        self.gem.animate(elapsed);
        self.cat.animate(elapsed);
        self
    }
}

#[derive(Show)]
pub struct Assets<T> {
    collect: Image<T>,
    avoid: Image<T>,
    gem: Sprite<T>,
    coin: Sprite<T>,
    cat: Sprite<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<F, AM>(font: &F, asset_manager: &mut AM) -> Result<Self>
    where
        F: Font<Texture = T>,
        AM: asset::Manager<Texture = T>,
    {
        let color = ColorRGBA(255, 255, 0, 255);

        //Collect
        let collect = font.texturize("Collect", &color)?
            .at(align::top(400).center(320));
        let distance = 50;
        let coin = {
            let pos = align::top(525).right(320 - distance / 2);
            asset_manager.sprite(asset::Animation::Coin, pos)?.scale(2)
        };
        let gem = {
            let pos = align::top(525).left(320 + distance / 2);
            asset_manager.sprite(asset::Animation::Gem, pos)?.scale(2)
        };

        //Avoid
        let avoid = font.texturize("Avoid", &color)?
            .at(align::top(400).center(960));
        let cat = {
            let pos = align::top(500).center(960);
            asset_manager
                .sprite(asset::Animation::IdleCat, pos)?
                .scale(2)
        };
        Ok(Assets {
            collect,
            gem,
            coin,
            avoid,
            cat,
        })
    }
}

impl<T> Assets<T> {
    pub fn next(mut self, world: &Guide) -> Self {
        self.gem.tile = world.gem.frame();
        self.coin.tile = world.coin.frame();
        self.cat.tile = world.cat.frame();
        self
    }
}
