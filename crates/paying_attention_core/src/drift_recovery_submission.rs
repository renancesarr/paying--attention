use crate::DeclaredTask;

/// A quick classification of what pulled attention away from the Focus Cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftCategory {
    OfflineDistraction,
    LinkHopping,
    Other,
}

/// The conscious choice made after acknowledging a drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftRecoveryAction {
    Retake,
    Restart,
    MarkIncomplete,
    NewTask(DeclaredTask),
}

/// The user's account of a detected attention drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftRecoverySubmission {
    pub note: String,
    pub category: DriftCategory,
    pub action: DriftRecoveryAction,
}

/// An accepted Drift Recovery submission retained for persistence adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftRecoveryRecord {
    pub drifted_task: DeclaredTask,
    pub submission: DriftRecoverySubmission,
}
