//! Configuration model for Paying Attention.
//!
//! This crate models configuration data without reading user directories.

use std::path::{Path, PathBuf};

pub const BUNDLED_NAGGING_SOUND_PATH: &str = "sounds/nagging.mp3";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AppConfig {
    pub timers: TimerConfig,
    pub nagging: NaggingConfig,
    pub telegram: Option<TelegramConfig>,
    pub meeting: MeetingConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaggingVisualStyle {
    DarkWhitePulse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub database_file: PathBuf,
}

impl XdgPaths {
    pub fn from_data_base(data_base: impl AsRef<Path>) -> Self {
        Self {
            database_file: data_base
                .as_ref()
                .join("paying-attention")
                .join("paying-attention.sqlite"),
        }
    }
}
