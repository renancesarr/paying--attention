use paying_attention_cli::manual_validation_report::render;
use paying_attention_storage::{ManualValidationEvidence, ManualValidationOutcome};

#[test]
fn report_renders_sqlite_evidence_as_a_markdown_table() {
    let evidence = ManualValidationEvidence {
        ticket_number: 10,
        check_name: "fullscreen-on-both-displays".into(),
        outcome: ManualValidationOutcome::Passed,
        observed_at: "2026-09-13T13:20:00-03:00".into(),
        command: "cargo run -p paying_attention_desktop".into(),
        observation: "Attention Block opened fullscreen on both displays.".into(),
        artifact_path: Some("/tmp/paying-attention-evidence/fullscreen.png".into()),
    };

    assert_eq!(
        render(10, &[evidence]),
        "# Manual Validation: Ticket 10\n\n| Check | Outcome | Observed at | Command | Observation | Artifact |\n| --- | --- | --- | --- | --- | --- |\n| fullscreen-on-both-displays | passed | 2026-09-13T13:20:00-03:00 | `cargo run -p paying_attention_desktop` | Attention Block opened fullscreen on both displays. | /tmp/paying-attention-evidence/fullscreen.png |\n"
    );
}

#[test]
fn report_escapes_table_characters_from_human_observations() {
    let evidence = ManualValidationEvidence {
        ticket_number: 10,
        check_name: "renderer".into(),
        outcome: ManualValidationOutcome::Blocked,
        observed_at: "2026-09-13T13:20:00-03:00".into(),
        command: "command | tee log.txt".into(),
        observation: "First line\nSecond | line".into(),
        artifact_path: None,
    };

    let report = render(10, &[evidence]);

    assert!(report.contains("`command \\| tee log.txt`"));
    assert!(report.contains("First line Second \\| line"));
}
