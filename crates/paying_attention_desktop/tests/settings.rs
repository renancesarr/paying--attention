use paying_attention_config::{AppConfig, NaggingVisualStyle};
use paying_attention_desktop::settings::SettingsInput;

#[test]
fn settings_input_updates_common_mvp_configuration() {
    let input = SettingsInput {
        boot_delay_minutes: 5,
        focus_cycle_minutes: 25,
        attention_block_idle_minutes: 3,
        focus_cycle_idle_minutes: 8,
        nagging_sound_path: "/tmp/alert.mp3".into(),
        telegram_bot_token: "token".into(),
        telegram_chat_id: "chat".into(),
        meeting_default_duration_minutes: 30,
    };

    let config = input
        .apply_to(AppConfig::default())
        .expect("valid settings");
    assert_eq!(config.timers.focus_cycle_minutes, 25);
    assert_eq!(
        config.nagging.sound_path(),
        std::path::Path::new("/tmp/alert.mp3")
    );
    assert_eq!(
        config.nagging.visual_style,
        NaggingVisualStyle::DarkWhitePulse
    );
    assert_eq!(
        config.telegram.expect("telegram configured").chat_id,
        "chat"
    );
    assert_eq!(config.meeting.default_duration_minutes, 30);
}

#[test]
fn settings_input_displays_the_current_configuration() {
    let input = SettingsInput::from_config(&AppConfig::default());

    assert_eq!(input.boot_delay_minutes, 10);
    assert_eq!(input.nagging_sound_path, "");
    assert_eq!(input.telegram_bot_token, "");
    assert_eq!(input.meeting_default_duration_minutes, 60);
}
