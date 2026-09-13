use crate::{ContinuationStatus, DeclaredTask, DriftRecoveryRecord, ReviewRecord};

/// The workflow phase that triggered Nagging Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaggingOrigin {
    CheckIn,
    Review,
    Focus,
}

/// Read-only information an adapter may use to render or persist the workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowView {
    Boot,
    CheckIn {
        last_drift_recovery: Option<DriftRecoveryRecord>,
    },
    Focus {
        declared_task: DeclaredTask,
        last_drift_recovery: Option<DriftRecoveryRecord>,
        last_review: Option<ReviewRecord>,
    },
    Review {
        declared_task: DeclaredTask,
        continuation: ContinuationStatus,
    },
    Nagging {
        origin: NaggingOrigin,
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
