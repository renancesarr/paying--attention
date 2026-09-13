use crate::DeclaredTask;

#[derive(Clone)]
pub(super) enum WorkflowState {
    Boot,
    CheckIn,
    NaggingDuringCheckIn,
    Focus { declared_task: DeclaredTask },
    Review { declared_task: DeclaredTask },
    NaggingDuringReview { declared_task: DeclaredTask },
    NaggingDuringFocus { declared_task: DeclaredTask },
    DriftRecovery { declared_task: DeclaredTask },
    MeetingMode { declared_task: DeclaredTask },
    MeetingEnd { declared_task: DeclaredTask },
}
