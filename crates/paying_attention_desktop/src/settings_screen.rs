use std::{cell::RefCell, rc::Rc};

use libadwaita::gtk::{self, prelude::*};
use paying_attention_config::AppConfig;

use crate::{settings::SettingsInput, strings};

pub fn build(config: AppConfig, on_save: impl Fn(AppConfig) + 'static) -> gtk::Box {
    let input = Rc::new(RefCell::new(SettingsInput::from_config(&config)));
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .build();
    let entries = [
        entry(
            strings::BOOT_DELAY,
            input.borrow().boot_delay_minutes.to_string(),
        ),
        entry(
            strings::FOCUS_DURATION,
            input.borrow().focus_cycle_minutes.to_string(),
        ),
        entry(
            strings::ATTENTION_BLOCK_IDLE,
            input.borrow().attention_block_idle_minutes.to_string(),
        ),
        entry(
            strings::FOCUS_IDLE,
            input.borrow().focus_cycle_idle_minutes.to_string(),
        ),
        entry(
            strings::NAGGING_SOUND,
            input.borrow().nagging_sound_path.clone(),
        ),
        entry(
            strings::TELEGRAM_TOKEN,
            input.borrow().telegram_bot_token.clone(),
        ),
        entry(
            strings::TELEGRAM_CHAT,
            input.borrow().telegram_chat_id.clone(),
        ),
        entry(
            strings::MEETING_DURATION,
            input.borrow().meeting_default_duration_minutes.to_string(),
        ),
    ];
    let save = gtk::Button::with_label(strings::SAVE_SETTINGS);
    for item in &entries {
        content.append(item);
    }
    let form = input.clone();
    save.connect_clicked(move |_| {
        let mut form = form.borrow_mut();
        form.boot_delay_minutes = entries[0].text().parse().unwrap_or(0);
        form.focus_cycle_minutes = entries[1].text().parse().unwrap_or(0);
        form.attention_block_idle_minutes = entries[2].text().parse().unwrap_or(0);
        form.focus_cycle_idle_minutes = entries[3].text().parse().unwrap_or(0);
        form.nagging_sound_path = entries[4].text().into();
        form.telegram_bot_token = entries[5].text().into();
        form.telegram_chat_id = entries[6].text().into();
        form.meeting_default_duration_minutes = entries[7].text().parse().unwrap_or(0);
        if let Some(updated) = form.apply_to(config.clone()) {
            on_save(updated);
        }
    });
    content.append(&save);
    content
}

fn entry(placeholder: &str, text: String) -> gtk::Entry {
    gtk::Entry::builder()
        .placeholder_text(placeholder)
        .text(text)
        .build()
}
