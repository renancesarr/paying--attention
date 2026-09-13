use crate::DeclaredTask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskRelevance {
    Relevant,
    Irrelevant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionStatus {
    Completed,
    NotCompleted,
    InProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewSubmission {
    pub relevance: TaskRelevance,
    pub completion: CompletionStatus,
    pub completion_justification: Option<String>,
    pub declared_task: DeclaredTask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRecord {
    pub reviewed_task: DeclaredTask,
    pub submission: ReviewSubmission,
}
