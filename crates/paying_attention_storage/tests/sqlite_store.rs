use paying_attention_storage::{AttentionHistoryEvent, FocusCycleRecord, SqliteAttentionStore};

#[test]
fn isolated_database_keeps_product_history_separate_from_technical_logs() {
    let mut store = SqliteAttentionStore::open_in_memory().expect("opens an isolated database");

    store
        .record_history(AttentionHistoryEvent::NaggingStarted)
        .expect("records product history");
    store
        .record_technical_log("Telegram request timed out")
        .expect("records a technical log");

    assert_eq!(
        store.history().expect("loads product history"),
        vec![AttentionHistoryEvent::NaggingStarted]
    );
    assert_eq!(
        store.technical_logs().expect("loads technical logs"),
        vec!["Telegram request timed out"]
    );
}

#[test]
fn product_history_survives_reopening_a_database() {
    let path = std::env::temp_dir().join(format!("paying-attention-{}.sqlite", std::process::id()));
    let _ = std::fs::remove_file(&path);

    {
        let mut store = SqliteAttentionStore::open(&path).expect("opens a test database");
        store
            .record_history(AttentionHistoryEvent::NaggingStarted)
            .expect("records product history");
    }

    let store = SqliteAttentionStore::open(&path).expect("reopens the test database");
    assert_eq!(
        store.history().expect("loads product history"),
        vec![AttentionHistoryEvent::NaggingStarted]
    );

    std::fs::remove_file(path).expect("removes the isolated test database");
}

#[test]
fn product_history_records_the_major_attention_events() {
    let mut store = SqliteAttentionStore::open_in_memory().expect("opens an isolated database");

    for event in [
        AttentionHistoryEvent::NaggingStarted,
        AttentionHistoryEvent::DriftRecoverySubmitted,
        AttentionHistoryEvent::MeetingModeStarted,
        AttentionHistoryEvent::TelegramFailed,
    ] {
        store
            .record_history(event)
            .expect("records a product event");
    }

    assert_eq!(
        store.history().expect("loads product history"),
        vec![
            AttentionHistoryEvent::NaggingStarted,
            AttentionHistoryEvent::DriftRecoverySubmitted,
            AttentionHistoryEvent::MeetingModeStarted,
            AttentionHistoryEvent::TelegramFailed,
        ]
    );
}

#[test]
fn restorable_state_replaces_the_previous_snapshot() {
    let mut store = SqliteAttentionStore::open_in_memory().expect("opens an isolated database");

    store
        .save_restorable_state("focus:implement-core")
        .expect("saves the first state");
    store
        .save_restorable_state("review:implement-core")
        .expect("replaces the state");

    assert_eq!(
        store.load_restorable_state().expect("loads state"),
        Some("review:implement-core".into())
    );
}

#[test]
fn focus_cycles_keep_the_declared_task_and_review_outcome() {
    let mut store = SqliteAttentionStore::open_in_memory().expect("opens an isolated database");

    store
        .record_focus_cycle(FocusCycleRecord {
            declared_task: "Implement the storage schema".into(),
            started_at: "2026-09-13T08:00:00-03:00".into(),
            ended_at: "2026-09-14T08:00:00-03:00".into(),
            relevance: Some("relevant".into()),
            completion: Some("in_progress".into()),
            completion_justification: Some("The persistence slice needs another cycle.".into()),
        })
        .expect("records a reviewed Focus Cycle");

    assert_eq!(
        store.focus_cycles().expect("loads Focus Cycles"),
        vec![FocusCycleRecord {
            declared_task: "Implement the storage schema".into(),
            started_at: "2026-09-13T08:00:00-03:00".into(),
            ended_at: "2026-09-14T08:00:00-03:00".into(),
            relevance: Some("relevant".into()),
            completion: Some("in_progress".into()),
            completion_justification: Some("The persistence slice needs another cycle.".into()),
        }]
    );
}
