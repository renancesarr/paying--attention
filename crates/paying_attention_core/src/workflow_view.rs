use crate::DeclaredTask;

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
    CheckIn,
    Focus { declared_task: DeclaredTask },
    Review { declared_task: DeclaredTask },
    Nagging { origin: NaggingOrigin },
    DriftRecovery { declared_task: DeclaredTask },
    MeetingMode { declared_task: DeclaredTask },
    MeetingEnd { declared_task: DeclaredTask },
}
