use std::path::PathBuf;

use paying_attention_config::{AppConfig, TelegramConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsInput {
    pub boot_delay_minutes: u16,
    pub focus_cycle_minutes: u16,
    pub attention_block_idle_minutes: u16,
    pub focus_cycle_idle_minutes: u16,
    pub nagging_sound_path: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub meeting_default_duration_minutes: u16,
}

impl SettingsInput {
    pub fn from_config(config: &AppConfig) -> Self {
        let telegram = config.telegram.as_ref();
        Self {
            boot_delay_minutes: config.timers.boot_delay_minutes,
            focus_cycle_minutes: config.timers.focus_cycle_minutes,
            attention_block_idle_minutes: config.timers.attention_block_idle_minutes,
            focus_cycle_idle_minutes: config.timers.focus_cycle_idle_minutes,
            nagging_sound_path: config
                .nagging
                .custom_sound_path
                .as_ref()
                .map_or_else(String::new, |path| path.display().to_string()),
            telegram_bot_token: telegram
                .map_or_else(String::new, |telegram| telegram.bot_token.clone()),
            telegram_chat_id: telegram
                .map_or_else(String::new, |telegram| telegram.chat_id.clone()),
            meeting_default_duration_minutes: config.meeting.default_duration_minutes,
        }
    }

    pub fn apply_to(&self, mut config: AppConfig) -> Option<AppConfig> {
        if !matches!(self.meeting_default_duration_minutes, 30 | 60 | 90) {
            return None;
        }

        config.timers.boot_delay_minutes = self.boot_delay_minutes;
        config.timers.focus_cycle_minutes = self.focus_cycle_minutes;
        config.timers.attention_block_idle_minutes = self.attention_block_idle_minutes;
        config.timers.focus_cycle_idle_minutes = self.focus_cycle_idle_minutes;
        config.nagging.custom_sound_path = (!self.nagging_sound_path.trim().is_empty())
            .then(|| PathBuf::from(self.nagging_sound_path.trim()));
        config.telegram = (!self.telegram_bot_token.trim().is_empty()
            && !self.telegram_chat_id.trim().is_empty())
        .then(|| TelegramConfig {
            bot_token: self.telegram_bot_token.trim().into(),
            chat_id: self.telegram_chat_id.trim().into(),
        });
        config.meeting.default_duration_minutes = self.meeting_default_duration_minutes;
        Some(config)
    }
}
