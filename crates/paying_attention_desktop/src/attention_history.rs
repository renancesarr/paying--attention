use paying_attention_storage::{AttentionHistoryEvent, FocusCycleRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryItem {
    FocusCycle { record: FocusCycleRecord },
    NaggingStarted,
    DriftRecoverySubmitted,
    MeetingModeStarted,
    TelegramFailed,
}

pub struct HistoryView {
    pub items: Vec<HistoryItem>,
}

impl HistoryView {
    pub fn from_records(cycles: Vec<FocusCycleRecord>, events: Vec<AttentionHistoryEvent>) -> Self {
        let mut items = cycles
            .into_iter()
            .map(|record| HistoryItem::FocusCycle { record })
            .collect::<Vec<_>>();
        items.extend(events.into_iter().map(HistoryItem::from));
        Self { items }
    }
}

impl From<AttentionHistoryEvent> for HistoryItem {
    fn from(event: AttentionHistoryEvent) -> Self {
        match event {
            AttentionHistoryEvent::NaggingStarted => Self::NaggingStarted,
            AttentionHistoryEvent::DriftRecoverySubmitted => Self::DriftRecoverySubmitted,
            AttentionHistoryEvent::MeetingModeStarted => Self::MeetingModeStarted,
            AttentionHistoryEvent::TelegramFailed => Self::TelegramFailed,
        }
    }
}
