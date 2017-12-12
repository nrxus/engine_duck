use asset::{self, Sprite};
use data::{self, Animators};
use game;

use moho::animation::animator::{self, Animator};
use moho::animation::TileSheet;
use moho::engine::{NextScene, World};
use moho::errors::*;
use moho::{self, input};
use moho::renderer::{align, Draw, Renderer, Show};
use moho::texture::{Image, Texture};
use sdl2::keyboard::Keycode;

use std::rc::Rc;
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
}

impl World for Gui {
    type Quit = ButtonKind;

    fn update(mut self, input: &input::State, elapsed: Duration) -> game::State<Self> {
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
    Selected(Sprite<T>, Rc<T>),
    Idle(Image<T>, TileSheet<T>),
}

impl<T> Button<T> {
    fn deselected(self) -> Self {
        match self {
            Button::Selected(s, texture) => Button::Idle(
                Image {
                    texture,
                    dst: s.dst,
                },
                s.sheet,
            ),
            b @ _ => b,
        }
    }

    fn selected(self, tile: u32) -> Self {
        match self {
            Button::Selected(mut s, t) => {
                s.tile = tile;
                Button::Selected(s, t)
            }
            Button::Idle(i, sheet) => Button::Selected(
                Sprite {
                    sheet,
                    tile,
                    dst: i.dst,
                },
                i.texture,
            ),
        }
    }
}

pub struct Assets<T> {
    duck: Button<T>,
    husky: Button<T>,
}

impl<T: Texture> Assets<T> {
    pub fn load<AM>(asset_manager: &mut AM, data: &data::Game) -> Result<Self>
    where
        AM: asset::Manager<Texture = T>,
    {
        let distance = 50;
        let husky = {
            let data = &data.husky;
            let texture = asset_manager.texture(&data.idle_texture)?;
            let pos = align::right(640 - distance / 2).bottom(300);
            let dst = data.out_size.dst(pos).scale(2);
            let image = Image { texture, dst };
            let sheet = asset_manager.animation(&data.animation)?;
            Button::Idle(image, sheet)
        };
        let duck = {
            let data = &data.duck;
            let texture = asset_manager.texture(&data.idle_texture)?;
            let pos = align::left(640 + distance / 2).bottom(300);
            let dst = data.out_size.dst(pos).scale(2);
            let image = Image { texture, dst };
            let sheet = asset_manager.animation(&data.animation)?;
            Button::Idle(image, sheet)
        };
        Ok(Self { duck, husky })
    }
}

impl<T: Draw<R>, R: Renderer> Show<R> for Button<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match *self {
            Button::Selected(ref s, _) => renderer.show(s),
            Button::Idle(ref i, _) => renderer.show(i),
        }
    }
}

impl<T: Draw<R>, R: Renderer> Show<R> for Assets<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.duck)?;
        renderer.show(&self.husky)
    }
}

impl<T> NextScene<Gui, (), ()> for Assets<T> {
    fn next(mut self, gui: &Gui, _: &(), _: &mut ()) -> Result<Self> {
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
        Ok(self)
    }
}
