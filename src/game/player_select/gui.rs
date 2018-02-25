use asset::{self, Sprite};
use data::Animators;
use Result;

use moho::animation::animator::{self, Animator};
use moho::animation::TileSheet;
use moho::{self, input};
use moho::renderer::{align, Draw, Renderer, Show};
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub enum ButtonKind {
    Husky,
    Duck,
}

enum Selected {
    Husky(Animator),
    Duck(Animator),
}

pub struct Gui {
    selected: Option<Selected>,
    duck: animator::Data,
    husky: animator::Data,
}

impl Gui {
    pub fn new(animators: &Animators) -> Self {
        Gui {
            selected: None,
            duck: animators.duck,
            husky: animators.husky,
        }
    }

    pub fn update(
        mut self,
        input: &input::State,
        elapsed: Duration,
    ) -> moho::State<Self, ButtonKind> {
        let (left, right) = {
            let left = input.did_press_key(Keycode::Left);
            let right = input.did_press_key(Keycode::Right);
            (left && !right, right && !left)
        };

        self.selected = match self.selected {
            Some(Selected::Husky(mut a)) if !right => {
                a.animate(elapsed);
                Some(Selected::Husky(a))
            }
            Some(Selected::Duck(mut a)) if !left => {
                a.animate(elapsed);
                Some(Selected::Duck(a))
            }
            _ if right => Some(Selected::Duck(self.duck.start())),
            _ if left => Some(Selected::Husky(self.husky.start())),
            _ => None,
        };

        match self.selected {
            Some(ref a) if input.did_press_key(Keycode::Return) => {
                let selected = match *a {
                    Selected::Husky(_) => ButtonKind::Husky,
                    Selected::Duck(_) => ButtonKind::Duck,
                };
                moho::State::Quit(selected)
            }
            _ => moho::State::Running(self),
        }
    }
}

enum Button<T> {
    Selected(Sprite<T>, T, Image<T>),
    Idle(Image<T>, TileSheet<T>, T),
}

impl<T: Texture> Button<T> {
    fn deselected(self) -> Self {
        match self {
            Button::Selected(s, texture, picker) => {
                let dst = s.dst;
                Button::Idle(Image { texture, dst }, s.sheet, picker.texture)
            }
            b => b,
        }
    }

    fn selected(self, tile: u32) -> Self {
        match self {
            Button::Selected(mut s, t, picker) => {
                s.tile = tile;
                Button::Selected(s, t, picker)
            }
            Button::Idle(i, sheet, picker) => {
                let dst = i.dst;
                let picker = {
                    let texture = picker;
                    let dst = align::top(dst.bottom() + 10)
                        .center(dst.center())
                        .dims(texture.dims());
                    Image { texture, dst }
                };

                Button::Selected(Sprite { sheet, tile, dst }, i.texture, picker)
            }
        }
    }
}

pub struct Assets<T> {
    duck: Button<T>,
    husky: Button<T>,
}

impl<T: Texture + Clone> Assets<T> {
    pub fn load<AM>(asset_manager: &mut AM) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let distance = 50;
        let picker = asset_manager.texture(asset::Texture::Heart)?;
        let husky = {
            let pos = align::right(640 - distance / 2).bottom(300);
            let mut image = asset_manager.image(asset::Texture::Husky, pos)?.scale(2);
            let sheet = asset_manager.sheet(asset::Animation::Husky)?;
            Button::Idle(image, sheet, picker.clone())
        };
        let duck = {
            let pos = align::left(640 + distance / 2).bottom(300);
            let mut image = asset_manager.image(asset::Texture::Duck, pos)?.scale(2);
            let sheet = asset_manager.sheet(asset::Animation::Duck)?;
            Button::Idle(image, sheet, picker)
        };
        Ok(Self { duck, husky })
    }

    pub fn next(mut self, gui: &Gui) -> Self {
        match gui.selected {
            None => {
                self.duck = self.duck.deselected();
                self.husky = self.husky.deselected();
            }
            Some(Selected::Duck(a)) => {
                self.husky = self.husky.deselected();
                self.duck = self.duck.selected(a.frame());
            }
            Some(Selected::Husky(a)) => {
                self.duck = self.duck.deselected();
                self.husky = self.husky.selected(a.frame());
            }
        }
        self
    }
}

impl<T: Draw<R>, R: Renderer> Show<R> for Button<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Button::Selected(ref s, _, ref p) => {
                renderer.show(s)?;
                renderer.show(p)
            }
            Button::Idle(ref i, _, _) => renderer.show(i),
        }
    }
}

impl<T: Draw<R>, R: Renderer> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.duck)?;
        renderer.show(&self.husky)
    }
}
