use paying_attention_cli::manual_validation_command::execute;

#[test]
fn record_command_persists_evidence_and_report_command_renders_it() {
    let database = std::env::temp_dir().join(format!(
        "paying-attention-validation-{}.sqlite",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&database);

    let record = vec![
        "record".into(),
        "--ticket".into(),
        "10".into(),
        "--check".into(),
        "alt-tab".into(),
        "--outcome".into(),
        "failed".into(),
        "--observed-at".into(),
        "2026-09-13T13:30:00-03:00".into(),
        "--command".into(),
        "cargo run -p paying_attention_desktop".into(),
        "--observation".into(),
        "Alt+Tab switched away from the Attention Block.".into(),
    ];

    assert_eq!(
        execute(&record, &database).expect("records manual validation"),
        "recorded"
    );

    let report = vec!["report".into(), "--ticket".into(), "10".into()];
    assert!(execute(&report, &database)
        .expect("renders manual validation report")
        .contains("| alt-tab | failed |"));

    std::fs::remove_file(database).expect("removes isolated validation database");
}
