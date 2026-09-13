use super::{AttentionWorkflow, WorkflowState};
use crate::{
    CompletionStatus, DriftRecoveryAction, DriftRecoveryRecord, Event, InvalidTransition,
    ReviewRecord, WorkflowView, CONTINUATION_LIMIT,
};

impl AttentionWorkflow {
    /// Apply a domain event and return the resulting read-only workflow view.
    #[allow(clippy::result_large_err)]
    pub fn dispatch(&mut self, event: Event) -> Result<WorkflowView, InvalidTransition> {
        let next_state = match (self.state.clone(), event) {
            (WorkflowState::Boot, Event::BootDelayElapsed) => WorkflowState::CheckIn {
                last_drift_recovery: None,
            },
            (
                WorkflowState::CheckIn {
                    last_drift_recovery,
                },
                Event::IdleThresholdElapsed,
            ) => WorkflowState::NaggingDuringCheckIn {
                last_drift_recovery,
            },
            (
                WorkflowState::NaggingDuringCheckIn {
                    last_drift_recovery,
                },
                Event::InputDetected,
            ) => WorkflowState::CheckIn {
                last_drift_recovery,
            },
            (
                WorkflowState::CheckIn {
                    last_drift_recovery,
                },
                Event::CheckInSubmitted { declared_task },
            ) => WorkflowState::Focus {
                declared_task,
                continuations_used: 0,
                last_drift_recovery,
                last_review: None,
            },
            (
                WorkflowState::Focus {
                    declared_task,
                    continuations_used,
                    ..
                },
                Event::FocusElapsed,
            ) => WorkflowState::Review {
                declared_task,
                continuations_used,
            },
            (
                WorkflowState::Review {
                    declared_task: previous_task,
                    ..
                },
                Event::ReviewSubmitted { submission },
            ) if submission.declared_task != previous_task
                && (submission.completion == CompletionStatus::Completed
                    || submission
                        .completion_justification
                        .as_deref()
                        .is_some_and(|text| !text.trim().is_empty())) =>
            {
                let next_task = submission.declared_task.clone();
                WorkflowState::Focus {
                    declared_task: next_task,
                    continuations_used: 0,
                    last_drift_recovery: None,
                    last_review: Some(ReviewRecord {
                        reviewed_task: previous_task,
                        submission,
                    }),
                }
            }
            (
                WorkflowState::Review {
                    declared_task,
                    continuations_used,
                },
                Event::ContinueDeclaredTask { submission },
            ) if continuations_used < CONTINUATION_LIMIT
                && submission.declared_task == declared_task
                && (submission.completion == CompletionStatus::Completed
                    || submission
                        .completion_justification
                        .as_deref()
                        .is_some_and(|text| !text.trim().is_empty())) =>
            {
                WorkflowState::Focus {
                    declared_task: declared_task.clone(),
                    continuations_used: continuations_used + 1,
                    last_drift_recovery: None,
                    last_review: Some(ReviewRecord {
                        reviewed_task: declared_task,
                        submission,
                    }),
                }
            }
            (
                WorkflowState::Review {
                    declared_task,
                    continuations_used,
                },
                Event::IdleThresholdElapsed,
            ) => WorkflowState::NaggingDuringReview {
                declared_task,
                continuations_used,
            },
            (
                WorkflowState::NaggingDuringReview {
                    declared_task,
                    continuations_used,
                },
                Event::InputDetected,
            ) => WorkflowState::Review {
                declared_task,
                continuations_used,
            },
            (WorkflowState::Focus { declared_task, .. }, Event::IdleThresholdElapsed) => {
                WorkflowState::NaggingDuringFocus { declared_task }
            }
            (WorkflowState::Focus { declared_task, .. }, Event::MeetingModeStarted) => {
                WorkflowState::MeetingMode { declared_task }
            }
            (WorkflowState::MeetingMode { declared_task }, Event::MeetingModeElapsed) => {
                WorkflowState::MeetingEnd { declared_task }
            }
            (WorkflowState::NaggingDuringFocus { declared_task }, Event::InputDetected) => {
                WorkflowState::DriftRecovery { declared_task }
            }
            (
                WorkflowState::DriftRecovery {
                    declared_task: drifted_task,
                },
                Event::DriftRecoverySubmitted { submission },
            ) if !submission.note.trim().is_empty() => match &submission.action {
                DriftRecoveryAction::Retake => WorkflowState::Focus {
                    declared_task: drifted_task.clone(),
                    continuations_used: 0,
                    last_drift_recovery: Some(DriftRecoveryRecord {
                        drifted_task,
                        submission,
                    }),
                    last_review: None,
                },
                DriftRecoveryAction::Restart => WorkflowState::Focus {
                    declared_task: drifted_task.clone(),
                    continuations_used: 0,
                    last_drift_recovery: Some(DriftRecoveryRecord {
                        drifted_task,
                        submission,
                    }),
                    last_review: None,
                },
                DriftRecoveryAction::MarkIncomplete => WorkflowState::CheckIn {
                    last_drift_recovery: Some(DriftRecoveryRecord {
                        drifted_task,
                        submission,
                    }),
                },
                DriftRecoveryAction::NewTask(declared_task) => WorkflowState::Focus {
                    declared_task: declared_task.clone(),
                    continuations_used: 0,
                    last_drift_recovery: Some(DriftRecoveryRecord {
                        drifted_task,
                        submission,
                    }),
                    last_review: None,
                },
            },
            (_, event) => {
                return Err(InvalidTransition {
                    state: self.view(),
                    event,
                });
            }
        };

        self.state = next_state;
        Ok(self.view())
    }
}
