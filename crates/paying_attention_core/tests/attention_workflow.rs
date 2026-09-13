use paying_attention_core::{
    AttentionWorkflow, DeclaredTask, EmptyDeclaredTask, Event, InvalidTransition, NaggingOrigin,
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

    assert_eq!(view, WorkflowView::Focus { declared_task });
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

    assert_eq!(view, WorkflowView::Review { declared_task });
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

    assert_eq!(review_view, WorkflowView::Review { declared_task });
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
