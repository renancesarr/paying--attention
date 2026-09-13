use paying_attention_core::{
    CompletionStatus, ContinuationStatus, DeclaredTask, Event, ReviewSubmission, TaskRelevance,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewChoice {
    NewTask,
    Continue,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewInput {
    pub relevance: Option<TaskRelevance>,
    pub completion: Option<CompletionStatus>,
    pub completion_justification: String,
    pub next_task_text: String,
    pub choice: Option<ReviewChoice>,
}

impl ReviewInput {
    pub fn submission(&self) -> Option<ReviewSubmission> {
        let declared_task = DeclaredTask::new(self.next_task_text.clone()).ok()?;
        self.submission_for(declared_task)
    }

    pub fn event(
        &self,
        previous_task: &DeclaredTask,
        continuation: ContinuationStatus,
    ) -> Option<Event> {
        match self.choice? {
            ReviewChoice::NewTask => {
                let submission = self.submission()?;
                (submission.declared_task != *previous_task)
                    .then_some(Event::ReviewSubmitted { submission })
            }
            ReviewChoice::Continue if continuation.used < continuation.limit => {
                Some(Event::ContinueDeclaredTask {
                    submission: self.submission_for(previous_task.clone())?,
                })
            }
            ReviewChoice::Continue => None,
        }
    }

    fn submission_for(&self, declared_task: DeclaredTask) -> Option<ReviewSubmission> {
        let relevance = self.relevance?;
        let completion = self.completion?;
        let justification = self.completion_justification.trim();

        if completion != CompletionStatus::Completed && justification.is_empty() {
            return None;
        }

        Some(ReviewSubmission {
            relevance,
            completion,
            completion_justification: (!justification.is_empty()).then(|| justification.into()),
            declared_task,
        })
    }
}
