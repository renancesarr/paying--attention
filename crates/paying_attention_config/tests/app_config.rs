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
fn xdg_database_path_is_derived_from_an_injected_data_base_without_touching_filesystem() {
    let paths = XdgPaths::from_data_base("/tmp/data");

    assert_eq!(
        paths.database_file,
        std::path::Path::new("/tmp/data/paying-attention/paying-attention.sqlite")
    );
}
