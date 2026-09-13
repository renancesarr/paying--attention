use super::{AttentionWorkflow, WorkflowState};

impl AttentionWorkflow {
    /// Start the attention workflow in the boot delay phase.
    pub fn boot() -> Self {
        Self {
            state: WorkflowState::Boot,
        }
    }
}
