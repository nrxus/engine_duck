use super::Dimension;
use crate::Result;

use std::fs::File;

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub enum GroundKind {
    Top,
    Middle,
}

#[derive(Debug, serde::Deserialize)]
pub enum CatKind {
    Idle,
    Moving(u32),
}

#[derive(Debug, serde::Deserialize)]
pub struct Cat {
    pub kind: CatKind,
    pub bottom_left: Dimension,
}

#[derive(Debug, serde::Deserialize)]
pub struct Obstacle {
    pub count: Dimension,
    pub bottom_left: Dimension,
}

#[derive(Debug, serde::Deserialize)]
pub struct Spike {
    pub count: u32,
    pub bottom_left: Dimension,
    #[serde(default)]
    pub left: Option<GroundKind>,
    #[serde(default)]
    pub right: Option<GroundKind>,
    #[serde(default)]
    pub bottom: Option<GroundKind>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Level {
    pub obstacles: Vec<Obstacle>,
    pub goal: Dimension,
    pub gems: Vec<Dimension>,
    pub coins: Vec<Dimension>,
    pub cats: Vec<Cat>,
    pub spikes: Vec<Spike>,
}

impl Level {
    pub fn load(path: &'static str) -> Result<Self> {
        let f = File::open(path)?;
        serde_yaml::from_reader(&f).map_err(Into::into)
    }
}
