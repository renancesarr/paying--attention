use std::{cell::RefCell, rc::Rc};

use libadwaita::gtk::{self, prelude::*};
use paying_attention_core::{DriftCategory, DriftRecoverySubmission};

use crate::{
    drift_recovery::{DriftRecoveryChoice, DriftRecoveryInput},
    strings,
};

pub fn build(on_submit: impl Fn(DriftRecoverySubmission) + 'static) -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .build();
    let form = Rc::new(RefCell::new(DriftRecoveryInput::default()));
    let submit = gtk::Button::with_label(strings::RECOVER_FOCUS);
    submit.set_sensitive(false);
    let next_task = gtk::Entry::builder()
        .placeholder_text(strings::NEXT_TASK)
        .sensitive(false)
        .build();

    content.append(&gtk::Label::new(Some(strings::DRIFT_RECOVERY)));
    let note = gtk::Entry::builder()
        .placeholder_text(strings::DRIFT_NOTE)
        .build();
    connect_text(&note, form.clone(), submit.clone(), |form, text| {
        form.note = text
    });
    content.append(&note);
    let next_task_for_action = next_task.clone();
    append_options(
        &content,
        strings::DRIFT_CATEGORY,
        [
            strings::OFFLINE_DISTRACTION,
            strings::LINK_HOPPING,
            strings::OTHER,
        ],
        form.clone(),
        submit.clone(),
        |index, form| {
            form.category = Some(
                [
                    DriftCategory::OfflineDistraction,
                    DriftCategory::LinkHopping,
                    DriftCategory::Other,
                ][index],
            );
        },
    );
    append_options(
        &content,
        strings::RECOVERY_ACTION,
        [
            strings::RETAKE,
            strings::RESTART,
            strings::MARK_INCOMPLETE,
            strings::DECLARE_NEW_TASK,
        ],
        form.clone(),
        submit.clone(),
        move |index, form| {
            form.choice = Some(
                [
                    DriftRecoveryChoice::Retake,
                    DriftRecoveryChoice::Restart,
                    DriftRecoveryChoice::MarkIncomplete,
                    DriftRecoveryChoice::NewTask,
                ][index],
            );
            next_task_for_action.set_sensitive(index == 3);
        },
    );
    connect_text(&next_task, form.clone(), submit.clone(), |form, text| {
        form.next_task_text = text;
    });
    content.append(&next_task);
    submit.connect_clicked(move |_| {
        if let Some(submission) = form.borrow().submission() {
            on_submit(submission);
        }
    });
    content.append(&submit);
    content
}

fn append_options<const N: usize, F>(
    content: &gtk::Box,
    title: &str,
    labels: [&str; N],
    form: Rc<RefCell<DriftRecoveryInput>>,
    submit: gtk::Button,
    update: F,
) where
    F: Fn(usize, &mut DriftRecoveryInput) + Clone + 'static,
{
    content.append(&gtk::Label::new(Some(title)));
    let buttons = radio_row(labels);
    for (index, button) in buttons.iter().enumerate() {
        let form = form.clone();
        let submit = submit.clone();
        let update = update.clone();
        button.connect_toggled(move |button| {
            if button.is_active() {
                update(index, &mut form.borrow_mut());
                submit.set_sensitive(form.borrow().submission().is_some());
            }
        });
        content.append(button);
    }
}

fn radio_row<const N: usize>(labels: [&str; N]) -> Vec<gtk::CheckButton> {
    let first = gtk::CheckButton::with_label(labels[0]);
    let mut buttons = vec![first.clone()];
    for label in labels.into_iter().skip(1) {
        let button = gtk::CheckButton::with_label(label);
        button.set_group(Some(&first));
        buttons.push(button);
    }
    buttons
}

fn connect_text(
    entry: &gtk::Entry,
    form: Rc<RefCell<DriftRecoveryInput>>,
    submit: gtk::Button,
    update: impl Fn(&mut DriftRecoveryInput, String) + 'static,
) {
    entry.connect_changed(move |entry| {
        update(&mut form.borrow_mut(), entry.text().into());
        submit.set_sensitive(form.borrow().submission().is_some());
    });
}
