use paying_attention_cli::TechnicalRecoveryService;
use paying_attention_storage::SqliteAttentionStore;

#[test]
fn status_reports_the_persisted_state_and_force_unlock_clears_it() {
    let mut store = SqliteAttentionStore::open_in_memory().expect("opens isolated storage");
    store
        .save_restorable_state("check_in")
        .expect("saves a blocking state");
    let mut service = TechnicalRecoveryService::new(&mut store);

    assert_eq!(service.status().expect("reads status"), "check_in");
    service.force_unlock().expect("clears the blocking state");
    assert_eq!(service.status().expect("reads cleared status"), "unlocked");
}
