use Result;

use failure::Error;
use serde_yaml;

use std::fs::File;

const PATH: &str = "media/high_scores.yaml";

#[derive(Debug, Deserialize, Serialize, Clone)]
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
