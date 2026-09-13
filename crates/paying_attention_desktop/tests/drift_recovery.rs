use paying_attention_core::{
    DeclaredTask, DriftCategory, DriftRecoveryAction, DriftRecoverySubmission,
};
use paying_attention_desktop::drift_recovery::{DriftRecoveryChoice, DriftRecoveryInput};

#[test]
fn recovery_requires_a_note_category_and_conscious_action() {
    let input = DriftRecoveryInput {
        note: "   ".into(),
        category: Some(DriftCategory::LinkHopping),
        choice: Some(DriftRecoveryChoice::Retake),
        ..Default::default()
    };

    assert_eq!(input.submission(), None);
}

#[test]
fn recovery_builds_a_submission_for_a_new_declared_task() {
    let input = DriftRecoveryInput {
        note: "Segui links fora da tarefa.".into(),
        category: Some(DriftCategory::LinkHopping),
        choice: Some(DriftRecoveryChoice::NewTask),
        next_task_text: "Registrar os links úteis".into(),
    };

    assert_eq!(
        input.submission(),
        Some(DriftRecoverySubmission {
            note: "Segui links fora da tarefa.".into(),
            category: DriftCategory::LinkHopping,
            action: DriftRecoveryAction::NewTask(
                DeclaredTask::new("Registrar os links úteis").expect("valid task"),
            ),
        })
    );
}
