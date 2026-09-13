use paying_attention_core::{
    AttentionWorkflow, DeclaredTask, EmptyDeclaredTask, Event, InvalidTransition, WorkflowView,
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
