use super::{AttentionWorkflow, WorkflowState};
use crate::{Event, InvalidTransition, WorkflowView};

impl AttentionWorkflow {
    /// Apply a domain event and return the resulting read-only workflow view.
    pub fn dispatch(&mut self, event: Event) -> Result<WorkflowView, InvalidTransition> {
        let next_state = match (self.state.clone(), event) {
            (WorkflowState::Boot, Event::BootDelayElapsed) => WorkflowState::CheckIn,
            (WorkflowState::CheckIn, Event::CheckInSubmitted { declared_task }) => {
                WorkflowState::Focus { declared_task }
            }
            (WorkflowState::Focus { declared_task }, Event::FocusElapsed) => {
                WorkflowState::Review { declared_task }
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
