use paying_attention_core::{
    DeclaredTask, DriftCategory, DriftRecoveryAction, DriftRecoverySubmission,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftRecoveryChoice {
    Retake,
    Restart,
    MarkIncomplete,
    NewTask,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DriftRecoveryInput {
    pub note: String,
    pub category: Option<DriftCategory>,
    pub choice: Option<DriftRecoveryChoice>,
    pub next_task_text: String,
}

impl DriftRecoveryInput {
    pub fn submission(&self) -> Option<DriftRecoverySubmission> {
        let note = self.note.trim();
        if note.is_empty() {
            return None;
        }

        let action = match self.choice? {
            DriftRecoveryChoice::Retake => DriftRecoveryAction::Retake,
            DriftRecoveryChoice::Restart => DriftRecoveryAction::Restart,
            DriftRecoveryChoice::MarkIncomplete => DriftRecoveryAction::MarkIncomplete,
            DriftRecoveryChoice::NewTask => {
                DriftRecoveryAction::NewTask(DeclaredTask::new(self.next_task_text.clone()).ok()?)
            }
        };

        Some(DriftRecoverySubmission {
            note: note.into(),
            category: self.category?,
            action,
        })
    }
}
