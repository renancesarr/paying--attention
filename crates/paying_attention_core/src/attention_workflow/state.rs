use crate::DeclaredTask;

#[derive(Clone)]
pub(super) enum WorkflowState {
    Boot,
    CheckIn,
    Focus { declared_task: DeclaredTask },
    Review { declared_task: DeclaredTask },
}
