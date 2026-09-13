use crate::{Event, WorkflowView};

/// A domain event was not valid for the current Attention Workflow view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidTransition {
    pub state: WorkflowView,
    pub event: Event,
}
