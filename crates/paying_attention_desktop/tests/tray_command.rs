use paying_attention_core::Event;
use paying_attention_desktop::tray_command::{MeetingModeInput, TrayCommand};

#[test]
fn meeting_command_requires_a_reason_and_supported_duration() {
    let input = MeetingModeInput {
        reason: "Daily sync".into(),
        duration_minutes: 60,
    };

    assert_eq!(
        input.command(),
        Some(TrayCommand::StartMeeting(Event::MeetingModeStarted {
            reason: "Daily sync".into(),
            duration_minutes: 60,
        }))
    );
}

#[test]
fn unsupported_meeting_request_does_not_create_a_tray_command() {
    let input = MeetingModeInput {
        reason: " ".into(),
        duration_minutes: 15,
    };

    assert_eq!(input.command(), None);
}
