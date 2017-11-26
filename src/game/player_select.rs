use moho::errors::*;

pub struct PlayerSelect {}

pub struct Assets {}

impl Assets {
    pub fn load() -> Result<Self> {
        Ok(Assets {})
    }
}
