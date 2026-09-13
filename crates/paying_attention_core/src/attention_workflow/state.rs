use crate::{DeclaredTask, DriftRecoveryRecord, ReviewRecord};

#[derive(Clone)]
pub(super) enum WorkflowState {
    Boot,
    CheckIn {
        last_drift_recovery: Option<DriftRecoveryRecord>,
    },
    NaggingDuringCheckIn {
        last_drift_recovery: Option<DriftRecoveryRecord>,
    },
    Focus {
        declared_task: DeclaredTask,
        continuations_used: u8,
        last_drift_recovery: Option<DriftRecoveryRecord>,
        last_review: Option<ReviewRecord>,
    },
    Review {
        declared_task: DeclaredTask,
        continuations_used: u8,
    },
    NaggingDuringReview {
        declared_task: DeclaredTask,
        continuations_used: u8,
    },
    NaggingDuringFocus {
        declared_task: DeclaredTask,
    },
    DriftRecovery {
        declared_task: DeclaredTask,
    },
    MeetingMode {
        declared_task: DeclaredTask,
        reason: String,
        duration_minutes: u16,
    },
    MeetingEnd {
        declared_task: DeclaredTask,
        reason: String,
        duration_minutes: u16,
    },
}
