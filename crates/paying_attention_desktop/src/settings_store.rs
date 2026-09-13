use std::{
    fs, io,
    path::{Path, PathBuf},
};

use paying_attention_config::AppConfig;

pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }

    pub fn load(&self) -> Result<AppConfig, Box<dyn std::error::Error>> {
        Ok(AppConfig::from_toml(&fs::read_to_string(&self.path)?)?)
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let parent = self
            .path
            .parent()
            .ok_or_else(|| io::Error::other("settings path has no parent"))?;
        fs::create_dir_all(parent)?;
        fs::write(&self.path, config.to_toml()?)?;
        Ok(())
    }
}
