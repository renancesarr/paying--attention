use paying_attention_storage::{
    ManualValidationEvidence, ManualValidationOutcome, ManualValidationStore,
};

#[test]
fn manual_validation_evidence_is_durable_and_queryable_by_ticket() {
    let mut store = ManualValidationStore::open_in_memory().expect("opens isolated evidence store");
    let evidence = ManualValidationEvidence {
        ticket_number: 10,
        check_name: "fullscreen-on-both-displays".into(),
        outcome: ManualValidationOutcome::Passed,
        observed_at: "2026-09-13T13:20:00-03:00".into(),
        command: "cargo run -p paying_attention_desktop".into(),
        observation: "Attention Block opened fullscreen on both displays.".into(),
        artifact_path: Some("/tmp/paying-attention-evidence/fullscreen.png".into()),
    };

    store
        .record_manual_validation(evidence.clone())
        .expect("records evidence");

    assert_eq!(
        store
            .manual_validation_evidence_for(10)
            .expect("loads evidence for ticket"),
        vec![evidence]
    );
    assert!(store
        .manual_validation_evidence_for(12)
        .expect("loads empty evidence for another ticket")
        .is_empty());
}
