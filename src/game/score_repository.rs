use crate::Result;

use failure::Error;

use std::fs::File;

const PATH: &str = "media/high_scores.yaml";

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Score {
    pub points: u32,
    pub name: String,
}

pub fn get() -> Vec<Score> {
    File::open(PATH)
        .map_err(Error::from)
        .and_then(|f| serde_yaml::from_reader(f).map_err(Into::into))
        .unwrap_or_default()
}

pub fn create(entries: &[Score]) -> Result<()> {
    let file = File::create(PATH)?;
    serde_yaml::to_writer(file, entries).map_err(Into::into)
}
