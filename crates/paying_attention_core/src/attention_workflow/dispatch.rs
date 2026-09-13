use super::{AttentionWorkflow, WorkflowState};
use crate::{Event, InvalidTransition, WorkflowView, CONTINUATION_LIMIT};

impl AttentionWorkflow {
    /// Apply a domain event and return the resulting read-only workflow view.
    pub fn dispatch(&mut self, event: Event) -> Result<WorkflowView, InvalidTransition> {
        let next_state = match (self.state.clone(), event) {
            (WorkflowState::Boot, Event::BootDelayElapsed) => WorkflowState::CheckIn,
            (WorkflowState::CheckIn, Event::IdleThresholdElapsed) => {
                WorkflowState::NaggingDuringCheckIn
            }
            (WorkflowState::NaggingDuringCheckIn, Event::InputDetected) => WorkflowState::CheckIn,
            (WorkflowState::CheckIn, Event::CheckInSubmitted { declared_task }) => {
                WorkflowState::Focus {
                    declared_task,
                    continuations_used: 0,
                }
            }
            (
                WorkflowState::Focus {
                    declared_task,
                    continuations_used,
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
                Event::ReviewSubmitted { declared_task },
            ) if declared_task != previous_task => WorkflowState::Focus {
                declared_task,
                continuations_used: 0,
            },
            (
                WorkflowState::Review {
                    declared_task,
                    continuations_used,
                },
                Event::ContinueDeclaredTask,
            ) if continuations_used < CONTINUATION_LIMIT => WorkflowState::Focus {
                declared_task,
                continuations_used: continuations_used + 1,
            },
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
