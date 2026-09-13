use paying_attention_core::Event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrayCommand {
    OpenSettings,
    OpenAttentionHistory,
    OpenMeetingMode,
    StartMeeting(Event),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeetingModeInput {
    pub reason: String,
    pub duration_minutes: u16,
}

impl MeetingModeInput {
    pub fn command(&self) -> Option<TrayCommand> {
        let reason = self.reason.trim();
        matches!(self.duration_minutes, 30 | 60 | 90)
            .then(|| {
                TrayCommand::StartMeeting(Event::MeetingModeStarted {
                    reason: reason.into(),
                    duration_minutes: self.duration_minutes,
                })
            })
            .filter(|_| !reason.is_empty())
    }
}
