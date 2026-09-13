use std::{cell::RefCell, rc::Rc};

use libadwaita::{self as adw, gtk, prelude::*};
use paying_attention_core::AttentionWorkflow;

use crate::{
    strings,
    tray_command::{MeetingModeInput, TrayCommand},
};

pub fn show(
    application: &adw::Application,
    workflow: Rc<RefCell<AttentionWorkflow>>,
    command: TrayCommand,
) {
    match command {
        TrayCommand::OpenSettings => present_message(application, strings::OPEN_SETTINGS),
        TrayCommand::OpenAttentionHistory => {
            present_message(application, strings::OPEN_ATTENTION_HISTORY)
        }
        TrayCommand::OpenMeetingMode => present_meeting_form(application, workflow),
        TrayCommand::StartMeeting(_) => {}
    }
}

fn present_message(application: &adw::Application, title: &str) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title(title)
        .default_width(420)
        .default_height(180)
        .build();
    window.set_content(Some(&gtk::Label::new(Some(title))));
    window.present();
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
