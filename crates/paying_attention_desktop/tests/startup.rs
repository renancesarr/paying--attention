use paying_attention_desktop::startup::{mode, StartupMode};

#[test]
fn autostart_uses_the_configured_boot_delay_while_manual_start_is_immediate() {
    assert_eq!(
        mode(&["--autostart".into()], 10),
        StartupMode::Autostart {
            boot_delay_minutes: 10
        }
    );
    assert_eq!(mode(&[], 10), StartupMode::Manual);
}
