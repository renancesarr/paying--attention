//! Configuration model for Paying Attention.
//!
//! This crate models configuration data without reading user directories.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const BUNDLED_NAGGING_SOUND_PATH: &str = "sounds/nagging.mp3";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub timers: TimerConfig,
    pub nagging: NaggingConfig,
    pub telegram: Option<TelegramConfig>,
    pub meeting: MeetingConfig,
}

impl AppConfig {
    pub fn from_toml(source: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(source)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerConfig {
    pub boot_delay_minutes: u16,
    pub focus_cycle_minutes: u16,
    pub attention_block_idle_minutes: u16,
    pub focus_cycle_idle_minutes: u16,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            boot_delay_minutes: 10,
            focus_cycle_minutes: 20,
            attention_block_idle_minutes: 2,
            focus_cycle_idle_minutes: 10,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaggingConfig {
    pub visual_style: NaggingVisualStyle,
    pub custom_sound_path: Option<PathBuf>,
}

impl NaggingConfig {
    pub fn sound_path(&self) -> &Path {
        self.custom_sound_path
            .as_deref()
            .unwrap_or_else(|| Path::new(BUNDLED_NAGGING_SOUND_PATH))
    }
}

impl Default for NaggingConfig {
    fn default() -> Self {
        Self {
            visual_style: NaggingVisualStyle::DarkWhitePulse,
            custom_sound_path: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NaggingVisualStyle {
    DarkWhitePulse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeetingConfig {
    pub default_duration_minutes: u16,
    pub allowed_durations_minutes: Vec<u16>,
}

impl Default for MeetingConfig {
    fn default() -> Self {
        Self {
            default_duration_minutes: 60,
            allowed_durations_minutes: vec![30, 60, 90],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdgPaths {
    pub config_file: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
}

impl XdgPaths {
    pub fn from_bases(
        config_base: impl AsRef<Path>,
        data_base: impl AsRef<Path>,
        state_base: impl AsRef<Path>,
    ) -> Self {
        Self {
            config_file: config_base
                .as_ref()
                .join("paying-attention")
                .join("config.toml"),
            data_dir: data_base.as_ref().join("paying-attention"),
            state_dir: state_base.as_ref().join("paying-attention"),
        }
    }
}
