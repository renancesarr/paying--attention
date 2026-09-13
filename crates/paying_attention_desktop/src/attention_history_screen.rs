use libadwaita::gtk::{self, prelude::*};

use crate::{
    attention_history::{HistoryItem, HistoryView},
    strings,
};

pub fn build(history: HistoryView) -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .build();
    content.append(&gtk::Label::new(Some(strings::ATTENTION_HISTORY)));
    for item in history.items {
        content.append(&gtk::Label::new(Some(&label(item))));
    }
    content
}

fn label(item: HistoryItem) -> String {
    match item {
        HistoryItem::FocusCycle { record } => format!(
            "{}: {} | {} - {} | {} | {}",
            strings::FOCUS_CYCLE,
            record.declared_task,
            record.started_at,
            record.ended_at,
            record
                .completion
                .unwrap_or_else(|| strings::NO_REVIEW.to_string()),
            record.completion_justification.unwrap_or_default(),
        ),
        HistoryItem::NaggingStarted => strings::HISTORY_NAGGING.into(),
        HistoryItem::DriftRecoverySubmitted => strings::HISTORY_DRIFT_RECOVERY.into(),
        HistoryItem::MeetingModeStarted => strings::HISTORY_MEETING.into(),
        HistoryItem::TelegramFailed => strings::HISTORY_TELEGRAM_FAILURE.into(),
    }
}
