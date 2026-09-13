mod boot;
mod dispatch;
mod state;
mod view;

use state::WorkflowState;

/// The pure domain module that owns attention phases and legal transitions.
pub struct AttentionWorkflow {
    state: WorkflowState,
}
