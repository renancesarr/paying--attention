use crate::DeclaredTask;

/// Read-only information an adapter may use to render or persist the workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowView {
    Boot,
    CheckIn,
    Focus { declared_task: DeclaredTask },
    Review { declared_task: DeclaredTask },
    Nagging,
    DriftRecovery,
    MeetingMode,
    MeetingEnd,
}
