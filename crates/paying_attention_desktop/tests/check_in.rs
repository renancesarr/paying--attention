use paying_attention_core::{DeclaredTask, Event};
use paying_attention_desktop::check_in::{CheckInInput, Energy, Environment};

#[test]
fn complete_check_in_releases_the_declared_task_to_the_core() {
    let input = CheckInInput {
        environment: Some(Environment::Home),
        energy: Some(Energy::Focused),
        task_text: "Implement the Check-in screen".into(),
    };

    assert!(input.can_release());
    assert_eq!(
        input.release_event(),
        Some(Event::CheckInSubmitted {
            declared_task: DeclaredTask::new("Implement the Check-in screen").expect("valid task"),
        })
    );
}

#[test]
fn incomplete_check_in_cannot_release_the_screen() {
    let input = CheckInInput {
        environment: Some(Environment::Office),
        energy: None,
        task_text: "  ".into(),
    };

    assert!(!input.can_release());
    assert_eq!(input.release_event(), None);
}
