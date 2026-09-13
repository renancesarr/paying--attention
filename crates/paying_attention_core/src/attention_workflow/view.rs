use super::{AttentionWorkflow, WorkflowState};
use crate::WorkflowView;

impl AttentionWorkflow {
    /// Read the workflow without exposing its internal representation.
    pub fn view(&self) -> WorkflowView {
        match &self.state {
            WorkflowState::Boot => WorkflowView::Boot,
            WorkflowState::CheckIn => WorkflowView::CheckIn,
            WorkflowState::Focus { declared_task } => WorkflowView::Focus {
                declared_task: declared_task.clone(),
            },
            WorkflowState::Review { declared_task } => WorkflowView::Review {
                declared_task: declared_task.clone(),
            },
        }
    }
}
