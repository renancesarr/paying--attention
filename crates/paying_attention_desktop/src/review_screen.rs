use std::{cell::RefCell, rc::Rc};

use libadwaita::gtk::{self, prelude::*};
use paying_attention_core::{
    CompletionStatus, ContinuationStatus, DeclaredTask, Event, TaskRelevance,
};

use crate::{
    review::{ReviewChoice, ReviewInput},
    strings,
};

#[derive(Clone)]
struct ReviewFormState {
    form: Rc<RefCell<ReviewInput>>,
    submit: gtk::Button,
    previous_task: DeclaredTask,
    continuation: ContinuationStatus,
}

pub fn build(
    previous_task: DeclaredTask,
    continuation: ContinuationStatus,
    on_submit: impl Fn(Event) + 'static,
) -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .build();
    let submit = gtk::Button::with_label(strings::START_NEXT_FOCUS);
    submit.set_sensitive(false);
    let state = ReviewFormState {
        form: Rc::new(RefCell::new(ReviewInput::default())),
        submit: submit.clone(),
        previous_task: previous_task.clone(),
        continuation,
    };
    let next_task = gtk::Entry::builder()
        .placeholder_text(strings::NEXT_TASK)
        .build();

    content.append(&gtk::Label::new(Some(strings::REVIEW)));
    content.append(&gtk::Label::new(Some(strings::PREVIOUS_TASK)));
    content.append(&gtk::Label::new(Some(previous_task.as_str())));
    append_question(
        &content,
        strings::RELEVANCE_QUESTION,
        radio_row([strings::RELEVANT, strings::IRRELEVANT]),
        state.clone(),
        |index, form| {
            form.relevance = Some([TaskRelevance::Relevant, TaskRelevance::Irrelevant][index]);
        },
    );
    append_question(
        &content,
        strings::COMPLETION_QUESTION,
        radio_row([
            strings::COMPLETED,
            strings::NOT_COMPLETED,
            strings::IN_PROGRESS,
        ]),
        state.clone(),
        |index, form| {
            form.completion = Some(
                [
                    CompletionStatus::Completed,
                    CompletionStatus::NotCompleted,
                    CompletionStatus::InProgress,
                ][index],
            );
        },
    );

    let justification = gtk::Entry::builder()
        .placeholder_text(strings::COMPLETION_JUSTIFICATION)
        .build();
    connect_text(&justification, state.clone(), |form, text| {
        form.completion_justification = text
    });
    content.append(&justification);

    let continuation_label = format!(
        "{} ({}/{})",
        strings::CONTINUE,
        continuation.used,
        continuation.limit
    );
    let task_choice = radio_row([strings::USE_NEW_TASK, &continuation_label]);
    task_choice[1].set_sensitive(continuation.used < continuation.limit);
    let next_task_for_choice = next_task.clone();
    connect_selection(&task_choice, state.clone(), move |index, form| {
        form.choice = Some([ReviewChoice::NewTask, ReviewChoice::Continue][index]);
        next_task_for_choice.set_sensitive(index == 0);
    });
    next_task.set_sensitive(false);
    connect_text(&next_task, state.clone(), |form, text| {
        form.next_task_text = text
    });
    content.append(&gtk::Label::new(Some(strings::NEXT_TASK)));
    append_all(&content, &task_choice);
    content.append(&next_task);

    submit.connect_clicked(move |_| {
        if let Some(event) = state
            .form
            .borrow()
            .event(&state.previous_task, state.continuation)
        {
            on_submit(event);
        }
    });
    content.append(&submit);
    content
}

fn append_question<F>(
    content: &gtk::Box,
    title: &str,
    buttons: Vec<gtk::CheckButton>,
    state: ReviewFormState,
    update: F,
) where
    F: Fn(usize, &mut ReviewInput) + Clone + 'static,
{
    content.append(&gtk::Label::new(Some(title)));
    connect_selection(&buttons, state, update);
    append_all(content, &buttons);
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

fn connect_selection<F>(buttons: &[gtk::CheckButton], state: ReviewFormState, update: F)
where
    F: Fn(usize, &mut ReviewInput) + Clone + 'static,
{
    for (index, button) in buttons.iter().enumerate() {
        let state = state.clone();
        let update = update.clone();
        button.connect_toggled(move |button| {
            if button.is_active() {
                update(index, &mut state.form.borrow_mut());
                state.submit.set_sensitive(
                    state
                        .form
                        .borrow()
                        .event(&state.previous_task, state.continuation)
                        .is_some(),
                );
            }
        });
    }
}

fn connect_text(
    entry: &gtk::Entry,
    state: ReviewFormState,
    update: impl Fn(&mut ReviewInput, String) + 'static,
) {
    entry.connect_changed(move |entry| {
        update(&mut state.form.borrow_mut(), entry.text().into());
        state.submit.set_sensitive(
            state
                .form
                .borrow()
                .event(&state.previous_task, state.continuation)
                .is_some(),
        );
    });
}

fn append_all(container: &gtk::Box, widgets: &[gtk::CheckButton]) {
    for widget in widgets {
        container.append(widget);
    }
}
