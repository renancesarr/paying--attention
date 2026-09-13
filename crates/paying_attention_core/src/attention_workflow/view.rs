use super::{AttentionWorkflow, WorkflowState};
use crate::{NaggingOrigin, WorkflowView};

impl AttentionWorkflow {
    /// Read the workflow without exposing its internal representation.
    pub fn view(&self) -> WorkflowView {
        match &self.state {
            WorkflowState::Boot => WorkflowView::Boot,
            WorkflowState::CheckIn => WorkflowView::CheckIn,
            WorkflowState::NaggingDuringCheckIn => WorkflowView::Nagging {
                origin: NaggingOrigin::CheckIn,
            },
            WorkflowState::Focus { declared_task } => WorkflowView::Focus {
                declared_task: declared_task.clone(),
            },
            WorkflowState::Review { declared_task } => WorkflowView::Review {
                declared_task: declared_task.clone(),
            },
            WorkflowState::NaggingDuringReview { .. } => WorkflowView::Nagging {
                origin: NaggingOrigin::Review,
            },
            WorkflowState::NaggingDuringFocus { .. } => WorkflowView::Nagging {
                origin: NaggingOrigin::Focus,
            },
            WorkflowState::DriftRecovery { declared_task } => WorkflowView::DriftRecovery {
                declared_task: declared_task.clone(),
            },
            WorkflowState::MeetingMode { declared_task } => WorkflowView::MeetingMode {
                declared_task: declared_task.clone(),
            },
            WorkflowState::MeetingEnd { declared_task } => WorkflowView::MeetingEnd {
                declared_task: declared_task.clone(),
            },
        }
    }
}
