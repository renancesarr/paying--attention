use paying_attention_config::{
    AppConfig, NaggingVisualStyle, XdgPaths, BUNDLED_NAGGING_SOUND_PATH,
};

#[test]
fn default_config_represents_the_mvp_attention_cadence() {
    let config = AppConfig::default();

    assert_eq!(config.timers.boot_delay_minutes, 10);
    assert_eq!(config.timers.focus_cycle_minutes, 20);
    assert_eq!(config.timers.attention_block_idle_minutes, 2);
    assert_eq!(config.timers.focus_cycle_idle_minutes, 10);
    assert_eq!(
        config.nagging.visual_style,
        NaggingVisualStyle::DarkWhitePulse
    );
    assert_eq!(config.nagging.sound_path(), BUNDLED_NAGGING_SOUND_PATH);
    assert_eq!(config.telegram, None);
    assert_eq!(config.meeting.default_duration_minutes, 60);
    assert_eq!(config.meeting.allowed_durations_minutes, vec![30, 60, 90]);
}

#[test]
fn representative_toml_round_trips_without_reading_user_directories() {
    let source = r#"
[timers]
boot_delay_minutes = 5
focus_cycle_minutes = 25
attention_block_idle_minutes = 3
focus_cycle_idle_minutes = 8

[nagging]
visual_style = "dark_white_pulse"
custom_sound_path = "/home/dudu/Music/attention.mp3"

[telegram]
bot_token = "bot-token"
chat_id = "123456"

[meeting]
default_duration_minutes = 30
allowed_durations_minutes = [30, 60, 90]
"#;

    let config = AppConfig::from_toml(source).expect("representative TOML is valid");

    assert_eq!(config.timers.focus_cycle_minutes, 25);
    assert_eq!(
        config.nagging.sound_path(),
        std::path::Path::new("/home/dudu/Music/attention.mp3")
    );
    assert_eq!(
        config
            .telegram
            .as_ref()
            .expect("Telegram is configured")
            .chat_id,
        "123456"
    );
    assert_eq!(config.meeting.default_duration_minutes, 30);
    assert_eq!(
        AppConfig::from_toml(&config.to_toml().expect("serializes")),
        Ok(config)
    );
}

#[test]
fn xdg_paths_are_derived_from_injected_bases_without_touching_the_filesystem() {
    let paths = XdgPaths::from_bases("/tmp/config", "/tmp/data", "/tmp/state");

    assert_eq!(
        paths.config_file,
        std::path::Path::new("/tmp/config/paying-attention/config.toml")
    );
    assert_eq!(
        paths.data_dir,
        std::path::Path::new("/tmp/data/paying-attention")
    );
    assert_eq!(
        paths.state_dir,
        std::path::Path::new("/tmp/state/paying-attention")
    );
}
