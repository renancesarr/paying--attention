use std::sync::mpsc::Sender;

use ksni::{blocking, menu::StandardItem, MenuItem, Tray};

use crate::{strings, tray_command::TrayCommand};

pub struct AttentionTray {
    commands: Sender<TrayCommand>,
}

impl AttentionTray {
    pub fn start(commands: Sender<TrayCommand>) -> Result<blocking::Handle<Self>, ksni::Error> {
        use blocking::TrayMethods;

        Self { commands }.assume_sni_available(true).spawn()
    }
}

impl Tray for AttentionTray {
    const MENU_ON_ACTIVATE: bool = true;

    fn id(&self) -> String {
        "paying-attention".into()
    }

    fn title(&self) -> String {
        strings::APP_TITLE.into()
    }

    fn icon_name(&self) -> String {
        "appointment-soon".into()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            item(
                strings::START_MEETING_MODE,
                self.commands.clone(),
                TrayCommand::OpenMeetingMode,
            ),
            item(
                strings::OPEN_SETTINGS,
                self.commands.clone(),
                TrayCommand::OpenSettings,
            ),
            item(
                strings::OPEN_ATTENTION_HISTORY,
                self.commands.clone(),
                TrayCommand::OpenAttentionHistory,
            ),
        ]
    }
}

fn item(
    label: &str,
    commands: Sender<TrayCommand>,
    command: TrayCommand,
) -> MenuItem<AttentionTray> {
    StandardItem {
        label: label.into(),
        activate: Box::new(move |_| {
            let _ = commands.send(command.clone());
        }),
        ..Default::default()
    }
    .into()
}
