use paying_attention_core::{
    CompletionStatus, ContinuationStatus, DeclaredTask, Event, ReviewSubmission, TaskRelevance,
};
use paying_attention_desktop::review::{ReviewChoice, ReviewInput};

#[test]
fn unfinished_review_requires_a_completion_justification() {
    let input = ReviewInput {
        relevance: Some(TaskRelevance::Relevant),
        completion: Some(CompletionStatus::NotCompleted),
        completion_justification: "".into(),
        next_task_text: "Implement the next screen".into(),
        ..Default::default()
    };

    assert_eq!(input.submission(), None);
}

#[test]
fn justified_review_builds_the_core_submission() {
    let input = ReviewInput {
        relevance: Some(TaskRelevance::Relevant),
        completion: Some(CompletionStatus::InProgress),
        completion_justification: "Needs another Focus Cycle.".into(),
        next_task_text: "Implement the next screen".into(),
        ..Default::default()
    };

    assert_eq!(
        input.submission(),
        Some(ReviewSubmission {
            relevance: TaskRelevance::Relevant,
            completion: CompletionStatus::InProgress,
            completion_justification: Some("Needs another Focus Cycle.".into()),
            declared_task: DeclaredTask::new("Implement the next screen").expect("valid task"),
        })
    );
}

#[test]
fn allowed_continuation_uses_the_previous_declared_task() {
    let previous_task = DeclaredTask::new("Implement the review screen").expect("valid task");
    let input = ReviewInput {
        relevance: Some(TaskRelevance::Relevant),
        completion: Some(CompletionStatus::InProgress),
        completion_justification: "Needs one more Focus Cycle.".into(),
        choice: Some(ReviewChoice::Continue),
        ..Default::default()
    };

    assert_eq!(
        input.event(&previous_task, ContinuationStatus { used: 1, limit: 2 }),
        Some(Event::ContinueDeclaredTask {
            submission: ReviewSubmission {
                relevance: TaskRelevance::Relevant,
                completion: CompletionStatus::InProgress,
                completion_justification: Some("Needs one more Focus Cycle.".into()),
                declared_task: previous_task,
            },
        })
    );
}

#[test]
fn exhausted_continuation_requires_a_different_next_task() {
    let previous_task = DeclaredTask::new("Implement the review screen").expect("valid task");
    let input = ReviewInput {
        relevance: Some(TaskRelevance::Relevant),
        completion: Some(CompletionStatus::InProgress),
        completion_justification: "Needs one more Focus Cycle.".into(),
        choice: Some(ReviewChoice::Continue),
        ..Default::default()
    };

    assert_eq!(
        input.event(&previous_task, ContinuationStatus { used: 2, limit: 2 }),
        None
    );
}
