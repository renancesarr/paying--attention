use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};

use libadwaita::{self as adw, gtk, prelude::*};
use paying_attention_core::AttentionWorkflow;
use paying_attention_storage::SqliteAttentionStore;

use crate::{
    attention_history::HistoryView,
    attention_history_screen, settings_screen,
    settings_store::SettingsStore,
    strings,
    tray_command::{MeetingModeInput, TrayCommand},
};

pub fn show(
    application: &adw::Application,
    workflow: Rc<RefCell<AttentionWorkflow>>,
    command: TrayCommand,
) {
    match command {
        TrayCommand::OpenSettings => present_settings(application),
        TrayCommand::OpenAttentionHistory => present_attention_history(application),
        TrayCommand::OpenMeetingMode => present_meeting_form(application, workflow),
        TrayCommand::StartMeeting(_) => {}
    }
}

fn present_attention_history(application: &adw::Application) {
    let path = data_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let history = SqliteAttentionStore::open(path)
        .map(|store| {
            HistoryView::from_records(
                store.focus_cycles().unwrap_or_default(),
                store.history().unwrap_or_default(),
            )
        })
        .unwrap_or(HistoryView { items: Vec::new() });
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title(strings::ATTENTION_HISTORY)
        .default_width(560)
        .default_height(420)
        .build();
    window.set_content(Some(&attention_history_screen::build(history)));
    window.present();
}

fn present_settings(application: &adw::Application) {
    let path = settings_path();
    let store = SettingsStore::new(&path);
    let config = store.load().unwrap_or_default();
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title(strings::OPEN_SETTINGS)
        .default_width(480)
        .default_height(500)
        .build();
    let window_for_save = window.clone();
    window.set_content(Some(&settings_screen::build(config, move |config| {
        let _ = SettingsStore::new(&path).save(&config);
        window_for_save.close();
    })));
    window.present();
}

fn settings_path() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("paying-attention/config.toml")
}

fn data_path() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from(".local/share"))
        .join("paying-attention/history.sqlite")
}

fn present_meeting_form(application: &adw::Application, workflow: Rc<RefCell<AttentionWorkflow>>) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title(strings::START_MEETING_MODE)
        .default_width(420)
        .default_height(240)
        .build();
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();
    let input = Rc::new(RefCell::new(MeetingModeInput {
        reason: String::new(),
        duration_minutes: 60,
    }));
    let reason = gtk::Entry::builder()
        .placeholder_text(strings::MEETING_REASON)
        .build();
    let start = gtk::Button::with_label(strings::START_MEETING_MODE);
    start.set_sensitive(false);
    let durations = duration_choices();

    connect_duration(&durations, input.clone(), start.clone());
    let form = input.clone();
    let button = start.clone();
    reason.connect_changed(move |entry| {
        form.borrow_mut().reason = entry.text().into();
        button.set_sensitive(form.borrow().command().is_some());
    });
    let window_for_submit = window.clone();
    start.connect_clicked(move |_| {
        if let Some(TrayCommand::StartMeeting(event)) = input.borrow().command() {
            if workflow.borrow_mut().dispatch(event).is_ok() {
                window_for_submit.close();
            }
        }
    });

    content.append(&gtk::Label::new(Some(strings::MEETING_REASON)));
    content.append(&reason);
    content.append(&gtk::Label::new(Some(strings::MEETING_DURATION)));
    for choice in durations {
        content.append(&choice);
    }
    content.append(&start);
    window.set_content(Some(&content));
    window.present();
}

fn duration_choices() -> Vec<gtk::CheckButton> {
    let first = gtk::CheckButton::with_label("30 minutos");
    let second = gtk::CheckButton::with_label("60 minutos");
    let third = gtk::CheckButton::with_label("90 minutos");
    second.set_group(Some(&first));
    third.set_group(Some(&first));
    second.set_active(true);
    vec![first, second, third]
}

fn connect_duration(
    choices: &[gtk::CheckButton],
    input: Rc<RefCell<MeetingModeInput>>,
    start: gtk::Button,
) {
    for (index, choice) in choices.iter().enumerate() {
        let input = input.clone();
        let start = start.clone();
        choice.connect_toggled(move |choice| {
            if choice.is_active() {
                input.borrow_mut().duration_minutes = [30, 60, 90][index];
                start.set_sensitive(input.borrow().command().is_some());
            }
        });
    }
}
