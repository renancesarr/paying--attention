use paying_attention_core::{
    AttentionWorkflow, CompletionStatus, ContinuationStatus, DeclaredTask, EmptyDeclaredTask,
    Event, InvalidTransition, NaggingOrigin, ReviewRecord, ReviewSubmission, TaskRelevance,
    WorkflowView,
};

#[test]
fn boot_delay_opens_check_in_through_the_attention_workflow() {
    let mut workflow = AttentionWorkflow::boot();

    let view = workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");

    assert_eq!(view, WorkflowView::CheckIn);
    assert_eq!(workflow.view(), WorkflowView::CheckIn);
}

#[test]
fn check_in_starts_a_focus_cycle_with_the_declared_task() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");

    let view = workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    assert_eq!(
        view,
        WorkflowView::Focus {
            declared_task,
            last_review: None,
        }
    );
}

#[test]
fn elapsed_focus_cycle_opens_review_with_the_original_declared_task() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    let view = workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    assert_eq!(
        view,
        WorkflowView::Review {
            declared_task,
            continuation: ContinuationStatus { used: 0, limit: 2 },
        }
    );
}

#[test]
fn review_with_a_new_declared_task_starts_the_next_focus_cycle() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let previous_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: previous_task.clone(),
        })
        .expect("a completed Check-in is valid");
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    let next_task = DeclaredTask::new("Write continuation rules").expect("a valid task");

    let submission = ReviewSubmission {
        relevance: TaskRelevance::Relevant,
        completion: CompletionStatus::Completed,
        completion_justification: None,
        declared_task: next_task.clone(),
    };
    let view = workflow
        .dispatch(Event::ReviewSubmitted {
            submission: submission.clone(),
        })
        .expect("a Review with a new task is valid");

    assert_eq!(
        view,
        WorkflowView::Focus {
            declared_task: next_task,
            last_review: Some(ReviewRecord {
                reviewed_task: previous_task,
                submission,
            }),
        }
    );
}

#[test]
fn review_rejects_an_unfinished_task_without_completion_justification() {
    let mut workflow =
        workflow_in_focus(DeclaredTask::new("Implement the core FSM").expect("a valid task"));
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    let next_task = DeclaredTask::new("Write the Review rules").expect("a valid task");

    assert_eq!(
        workflow.dispatch(Event::ReviewSubmitted {
            submission: ReviewSubmission {
                relevance: TaskRelevance::Relevant,
                completion: CompletionStatus::NotCompleted,
                completion_justification: None,
                declared_task: next_task,
            },
        }),
        Err(InvalidTransition {
            state: WorkflowView::Review {
                declared_task: DeclaredTask::new("Implement the core FSM").expect("a valid task"),
                continuation: ContinuationStatus { used: 0, limit: 2 },
            },
            event: Event::ReviewSubmitted {
                submission: ReviewSubmission {
                    relevance: TaskRelevance::Relevant,
                    completion: CompletionStatus::NotCompleted,
                    completion_justification: None,
                    declared_task: DeclaredTask::new("Write the Review rules")
                        .expect("a valid task"),
                },
            },
        })
    );
}

#[test]
fn review_rejects_an_in_progress_task_without_completion_justification() {
    let mut workflow =
        workflow_in_focus(DeclaredTask::new("Implement the core FSM").expect("a valid task"));
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    let result = workflow.dispatch(Event::ReviewSubmitted {
        submission: ReviewSubmission {
            relevance: TaskRelevance::Irrelevant,
            completion: CompletionStatus::InProgress,
            completion_justification: None,
            declared_task: DeclaredTask::new("Write the Review rules").expect("a valid task"),
        },
    });

    assert!(matches!(result, Err(InvalidTransition { .. })));
    assert_eq!(
        workflow.view(),
        WorkflowView::Review {
            declared_task: DeclaredTask::new("Implement the core FSM").expect("a valid task"),
            continuation: ContinuationStatus { used: 0, limit: 2 },
        }
    );
}

#[test]
fn review_accepts_an_in_progress_task_with_completion_justification() {
    let mut workflow =
        workflow_in_focus(DeclaredTask::new("Implement the core FSM").expect("a valid task"));
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    let next_task = DeclaredTask::new("Write the Review rules").expect("a valid task");

    let view = workflow
        .dispatch(Event::ReviewSubmitted {
            submission: ReviewSubmission {
                relevance: TaskRelevance::Relevant,
                completion: CompletionStatus::InProgress,
                completion_justification: Some("The implementation needs another cycle.".into()),
                declared_task: next_task.clone(),
            },
        })
        .expect("an in-progress task with a justification is valid");

    assert_eq!(
        view,
        WorkflowView::Focus {
            declared_task: next_task,
            last_review: Some(ReviewRecord {
                reviewed_task: DeclaredTask::new("Implement the core FSM").expect("a valid task"),
                submission: ReviewSubmission {
                    relevance: TaskRelevance::Relevant,
                    completion: CompletionStatus::InProgress,
                    completion_justification: Some(
                        "The implementation needs another cycle.".into()
                    ),
                    declared_task: DeclaredTask::new("Write the Review rules")
                        .expect("a valid task"),
                },
            }),
        }
    );
}

#[test]
fn review_accepts_an_unfinished_task_with_completion_justification() {
    let mut workflow =
        workflow_in_focus(DeclaredTask::new("Implement the core FSM").expect("a valid task"));
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    let next_task = DeclaredTask::new("Write the Review rules").expect("a valid task");

    let view = workflow
        .dispatch(Event::ReviewSubmitted {
            submission: ReviewSubmission {
                relevance: TaskRelevance::Irrelevant,
                completion: CompletionStatus::NotCompleted,
                completion_justification: Some("I lost the thread while investigating.".into()),
                declared_task: next_task.clone(),
            },
        })
        .expect("an unfinished task with a justification is valid");

    assert_eq!(
        view,
        WorkflowView::Focus {
            declared_task: next_task.clone(),
            last_review: Some(ReviewRecord {
                reviewed_task: DeclaredTask::new("Implement the core FSM").expect("a valid task"),
                submission: ReviewSubmission {
                    relevance: TaskRelevance::Irrelevant,
                    completion: CompletionStatus::NotCompleted,
                    completion_justification: Some("I lost the thread while investigating.".into()),
                    declared_task: next_task,
                },
            }),
        }
    );
}

#[test]
fn review_rejects_a_blank_completion_justification() {
    let mut workflow =
        workflow_in_focus(DeclaredTask::new("Implement the core FSM").expect("a valid task"));
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    let result = workflow.dispatch(Event::ReviewSubmitted {
        submission: ReviewSubmission {
            relevance: TaskRelevance::Relevant,
            completion: CompletionStatus::InProgress,
            completion_justification: Some("   ".into()),
            declared_task: DeclaredTask::new("Write the Review rules").expect("a valid task"),
        },
    });

    assert!(matches!(result, Err(InvalidTransition { .. })));
}

#[test]
fn review_can_continue_the_original_declared_task() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    let view = workflow
        .dispatch(Event::ContinueDeclaredTask {
            submission: completed_relevant_submission(declared_task.clone()),
        })
        .expect("the first Continuation is valid");

    assert_eq!(
        view,
        WorkflowView::Focus {
            declared_task: declared_task.clone(),
            last_review: Some(ReviewRecord {
                reviewed_task: declared_task.clone(),
                submission: completed_relevant_submission(declared_task),
            }),
        }
    );
}

#[test]
fn continuation_rejects_an_unfinished_review_without_completion_justification() {
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    let mut workflow = workflow_in_focus(declared_task.clone());
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    let result = workflow.dispatch(Event::ContinueDeclaredTask {
        submission: ReviewSubmission {
            relevance: TaskRelevance::Relevant,
            completion: CompletionStatus::NotCompleted,
            completion_justification: None,
            declared_task,
        },
    });

    assert!(matches!(result, Err(InvalidTransition { .. })));
}

#[test]
fn review_shows_one_used_continuation_after_the_first_continued_focus_cycle() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    workflow
        .dispatch(Event::ContinueDeclaredTask {
            submission: completed_relevant_submission(declared_task.clone()),
        })
        .expect("the first Continuation is valid");

    let view = workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed continued Focus Cycle is valid");

    assert_eq!(
        view,
        WorkflowView::Review {
            declared_task,
            continuation: ContinuationStatus { used: 1, limit: 2 },
        }
    );
}

#[test]
fn review_rejects_a_third_continuation_and_preserves_the_limit_state() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    for _ in 0..2 {
        workflow
            .dispatch(Event::FocusElapsed)
            .expect("an elapsed Focus Cycle is valid");
        workflow
            .dispatch(Event::ContinueDeclaredTask {
                submission: completed_relevant_submission(declared_task.clone()),
            })
            .expect("an available Continuation is valid");
    }
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed second continued Focus Cycle is valid");

    let expected_review = WorkflowView::Review {
        declared_task: declared_task.clone(),
        continuation: ContinuationStatus { used: 2, limit: 2 },
    };
    let rejected_continuation = Event::ContinueDeclaredTask {
        submission: completed_relevant_submission(declared_task),
    };

    assert_eq!(
        workflow.dispatch(rejected_continuation.clone()),
        Err(InvalidTransition {
            state: expected_review.clone(),
            event: rejected_continuation,
        })
    );
    assert_eq!(workflow.view(), expected_review);
}

#[test]
fn review_rejects_the_unchanged_task_after_the_continuation_limit() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    for _ in 0..2 {
        workflow
            .dispatch(Event::FocusElapsed)
            .expect("an elapsed Focus Cycle is valid");
        workflow
            .dispatch(Event::ContinueDeclaredTask {
                submission: completed_relevant_submission(declared_task.clone()),
            })
            .expect("an available Continuation is valid");
    }
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed second continued Focus Cycle is valid");

    let expected_review = WorkflowView::Review {
        declared_task: declared_task.clone(),
        continuation: ContinuationStatus { used: 2, limit: 2 },
    };

    assert_eq!(
        workflow.dispatch(Event::ReviewSubmitted {
            submission: completed_relevant_submission(declared_task),
        }),
        Err(InvalidTransition {
            state: expected_review.clone(),
            event: Event::ReviewSubmitted {
                submission: completed_relevant_submission(
                    DeclaredTask::new("Implement the core FSM").expect("a valid task"),
                ),
            },
        })
    );
    assert_eq!(workflow.view(), expected_review);
}

#[test]
fn review_resets_the_continuation_counter_for_a_new_declared_task() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let original_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: original_task.clone(),
        })
        .expect("a completed Check-in is valid");

    for _ in 0..2 {
        workflow
            .dispatch(Event::FocusElapsed)
            .expect("an elapsed Focus Cycle is valid");
        workflow
            .dispatch(Event::ContinueDeclaredTask {
                submission: completed_relevant_submission(original_task.clone()),
            })
            .expect("an available Continuation is valid");
    }
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed second continued Focus Cycle is valid");
    let next_task = DeclaredTask::new("Write the Review rules").expect("a valid task");
    workflow
        .dispatch(Event::ReviewSubmitted {
            submission: completed_relevant_submission(next_task.clone()),
        })
        .expect("a new Declared Task is valid after the limit");

    let view = workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle for the new task is valid");

    assert_eq!(
        view,
        WorkflowView::Review {
            declared_task: next_task,
            continuation: ContinuationStatus { used: 0, limit: 2 },
        }
    );
}

#[test]
fn idle_focus_cycle_enters_nagging_then_opens_drift_recovery_on_input() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    let nagging_view = workflow
        .dispatch(Event::IdleThresholdElapsed)
        .expect("idle Focus Cycle enters Nagging Mode");

    assert_eq!(
        nagging_view,
        WorkflowView::Nagging {
            origin: NaggingOrigin::Focus,
        }
    );

    let recovery_view = workflow
        .dispatch(Event::InputDetected)
        .expect("input after focus Nagging opens Drift Recovery");

    assert_eq!(recovery_view, WorkflowView::DriftRecovery { declared_task });
}

#[test]
fn idle_check_in_returns_to_check_in_when_input_is_detected() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");

    let nagging_view = workflow
        .dispatch(Event::IdleThresholdElapsed)
        .expect("idle Check-in enters Nagging Mode");

    assert_eq!(
        nagging_view,
        WorkflowView::Nagging {
            origin: NaggingOrigin::CheckIn,
        }
    );

    let check_in_view = workflow
        .dispatch(Event::InputDetected)
        .expect("input after Check-in Nagging returns to Check-in");

    assert_eq!(check_in_view, WorkflowView::CheckIn);
}

#[test]
fn idle_review_returns_to_review_with_its_declared_task_when_input_is_detected() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");
    workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");

    let nagging_view = workflow
        .dispatch(Event::IdleThresholdElapsed)
        .expect("idle Review enters Nagging Mode");

    assert_eq!(
        nagging_view,
        WorkflowView::Nagging {
            origin: NaggingOrigin::Review,
        }
    );

    let review_view = workflow
        .dispatch(Event::InputDetected)
        .expect("input after Review Nagging returns to Review");

    assert_eq!(
        review_view,
        WorkflowView::Review {
            declared_task,
            continuation: ContinuationStatus { used: 0, limit: 2 },
        }
    );
}

#[test]
fn meeting_mode_elapsed_opens_meeting_end_from_a_focus_cycle() {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");
    workflow
        .dispatch(Event::CheckInSubmitted {
            declared_task: declared_task.clone(),
        })
        .expect("a completed Check-in is valid");

    let meeting_view = workflow
        .dispatch(Event::MeetingModeStarted)
        .expect("Meeting Mode can start from a Focus Cycle");

    assert_eq!(
        meeting_view,
        WorkflowView::MeetingMode {
            declared_task: declared_task.clone()
        }
    );

    let meeting_end_view = workflow
        .dispatch(Event::MeetingModeElapsed)
        .expect("an elapsed Meeting Mode opens Meeting End");

    assert_eq!(meeting_end_view, WorkflowView::MeetingEnd { declared_task });
}

#[test]
fn invalid_events_preserve_each_extended_attention_workflow_view() {
    let declared_task = DeclaredTask::new("Implement the core FSM").expect("a valid task");

    let mut check_in_workflow = AttentionWorkflow::boot();
    check_in_workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    assert_invalid_transition(
        &mut check_in_workflow,
        Event::FocusElapsed,
        WorkflowView::CheckIn,
    );

    let mut focus_workflow = workflow_in_focus(declared_task.clone());
    assert_invalid_transition(
        &mut focus_workflow,
        Event::BootDelayElapsed,
        WorkflowView::Focus {
            declared_task: declared_task.clone(),
            last_review: None,
        },
    );

    let mut review_workflow = workflow_in_focus(declared_task.clone());
    review_workflow
        .dispatch(Event::FocusElapsed)
        .expect("an elapsed Focus Cycle is valid");
    assert_invalid_transition(
        &mut review_workflow,
        Event::MeetingModeElapsed,
        WorkflowView::Review {
            declared_task: declared_task.clone(),
            continuation: ContinuationStatus { used: 0, limit: 2 },
        },
    );

    let mut nagging_workflow = workflow_in_focus(declared_task.clone());
    nagging_workflow
        .dispatch(Event::IdleThresholdElapsed)
        .expect("idle Focus Cycle enters Nagging Mode");
    assert_invalid_transition(
        &mut nagging_workflow,
        Event::FocusElapsed,
        WorkflowView::Nagging {
            origin: NaggingOrigin::Focus,
        },
    );

    let mut recovery_workflow = workflow_in_focus(declared_task.clone());
    recovery_workflow
        .dispatch(Event::IdleThresholdElapsed)
        .expect("idle Focus Cycle enters Nagging Mode");
    recovery_workflow
        .dispatch(Event::InputDetected)
        .expect("input after focus Nagging opens Drift Recovery");
    assert_invalid_transition(
        &mut recovery_workflow,
        Event::MeetingModeElapsed,
        WorkflowView::DriftRecovery {
            declared_task: declared_task.clone(),
        },
    );

    let mut meeting_workflow = workflow_in_focus(declared_task.clone());
    meeting_workflow
        .dispatch(Event::MeetingModeStarted)
        .expect("Meeting Mode can start from a Focus Cycle");
    assert_invalid_transition(
        &mut meeting_workflow,
        Event::FocusElapsed,
        WorkflowView::MeetingMode {
            declared_task: declared_task.clone(),
        },
    );
    meeting_workflow
        .dispatch(Event::MeetingModeElapsed)
        .expect("an elapsed Meeting Mode opens Meeting End");
    assert_invalid_transition(
        &mut meeting_workflow,
        Event::FocusElapsed,
        WorkflowView::MeetingEnd { declared_task },
    );
}

#[test]
fn declared_task_rejects_blank_text() {
    assert_eq!(DeclaredTask::new("   "), Err(EmptyDeclaredTask));
}

#[test]
fn invalid_event_preserves_the_attention_workflow_view() {
    let mut workflow = AttentionWorkflow::boot();

    let result = workflow.dispatch(Event::FocusElapsed);

    assert_eq!(
        result,
        Err(InvalidTransition {
            state: WorkflowView::Boot,
            event: Event::FocusElapsed,
        })
    );
    assert_eq!(workflow.view(), WorkflowView::Boot);
}

fn workflow_in_focus(declared_task: DeclaredTask) -> AttentionWorkflow {
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("boot delay is valid");
    workflow
        .dispatch(Event::CheckInSubmitted { declared_task })
        .expect("a completed Check-in is valid");
    workflow
}

fn completed_relevant_submission(declared_task: DeclaredTask) -> ReviewSubmission {
    ReviewSubmission {
        relevance: TaskRelevance::Relevant,
        completion: CompletionStatus::Completed,
        completion_justification: None,
        declared_task,
    }
}

fn assert_invalid_transition(
    workflow: &mut AttentionWorkflow,
    event: Event,
    expected_view: WorkflowView,
) {
    assert_eq!(
        workflow.dispatch(event.clone()),
        Err(InvalidTransition {
            state: expected_view.clone(),
            event,
        })
    );
    assert_eq!(workflow.view(), expected_view);
}
