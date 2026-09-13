#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupMode {
    Manual,
    Autostart { boot_delay_minutes: u16 },
}

pub fn mode(arguments: &[String], boot_delay_minutes: u16) -> StartupMode {
    if arguments.iter().any(|argument| argument == "--autostart") {
        StartupMode::Autostart { boot_delay_minutes }
    } else {
        StartupMode::Manual
    }
}
