use crate::{DeclaredTask, ReviewRecord};

#[derive(Clone)]
pub(super) enum WorkflowState {
    Boot,
    CheckIn,
    NaggingDuringCheckIn,
    Focus {
        declared_task: DeclaredTask,
        continuations_used: u8,
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
    },
    MeetingEnd {
        declared_task: DeclaredTask,
    },
}
