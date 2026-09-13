use super::{AttentionWorkflow, WorkflowState};
use crate::{ContinuationStatus, NaggingOrigin, WorkflowView, CONTINUATION_LIMIT};

impl AttentionWorkflow {
    /// Read the workflow without exposing its internal representation.
    pub fn view(&self) -> WorkflowView {
        match &self.state {
            WorkflowState::Boot => WorkflowView::Boot,
            WorkflowState::CheckIn {
                last_drift_recovery,
            } => WorkflowView::CheckIn {
                last_drift_recovery: last_drift_recovery.clone(),
            },
            WorkflowState::NaggingDuringCheckIn { .. } => WorkflowView::Nagging {
                origin: NaggingOrigin::CheckIn,
            },
            WorkflowState::Focus {
                declared_task,
                last_drift_recovery,
                last_review,
                ..
            } => WorkflowView::Focus {
                declared_task: declared_task.clone(),
                last_drift_recovery: last_drift_recovery.clone(),
                last_review: last_review.clone(),
            },
            WorkflowState::Review {
                declared_task,
                continuations_used,
            } => WorkflowView::Review {
                declared_task: declared_task.clone(),
                continuation: ContinuationStatus {
                    used: *continuations_used,
                    limit: CONTINUATION_LIMIT,
                },
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
            WorkflowState::MeetingMode {
                declared_task,
                reason,
                duration_minutes,
            } => WorkflowView::MeetingMode {
                declared_task: declared_task.clone(),
                reason: reason.clone(),
                duration_minutes: *duration_minutes,
            },
            WorkflowState::MeetingEnd {
                declared_task,
                reason,
                duration_minutes,
            } => WorkflowView::MeetingEnd {
                declared_task: declared_task.clone(),
                reason: reason.clone(),
                duration_minutes: *duration_minutes,
            },
        }
    }
}
