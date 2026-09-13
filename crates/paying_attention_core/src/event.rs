use crate::{DeclaredTask, DriftRecoverySubmission, ReviewSubmission};

/// External input translated into domain events by adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The boot delay has elapsed.
    BootDelayElapsed,
    /// The current Focus Cycle has elapsed.
    FocusElapsed,
    /// The current state's idle threshold has elapsed.
    IdleThresholdElapsed,
    /// The user interacted with the keyboard or pointer.
    InputDetected,
    /// The user submitted the Check-in form.
    CheckInSubmitted { declared_task: DeclaredTask },
    /// The user submitted the Review form.
    ReviewSubmitted { submission: ReviewSubmission },
    /// The user submitted Review answers and chose to continue the Declared Task.
    ContinueDeclaredTask { submission: ReviewSubmission },
    /// The user submitted the Drift Recovery form.
    DriftRecoverySubmitted { submission: DriftRecoverySubmission },
    /// The user started Meeting Mode.
    MeetingModeStarted,
    /// The bounded Meeting Mode duration has elapsed.
    MeetingModeElapsed,
    /// The user submitted the Meeting End form.
    MeetingEndSubmitted,
}
