use paying_attention_desktop::attention_history::{HistoryItem, HistoryView};
use paying_attention_storage::{AttentionHistoryEvent, FocusCycleRecord};

#[test]
fn history_view_presents_cycles_and_product_events_without_technical_logs() {
    let view = HistoryView::from_records(
        vec![FocusCycleRecord {
            declared_task: "Implementar histórico".into(),
            started_at: "2026-09-13T08:00:00-03:00".into(),
            ended_at: "2026-09-13T20:00:00-03:00".into(),
            relevance: Some("relevante".into()),
            completion: Some("em andamento".into()),
            completion_justification: Some("Faltou tempo".into()),
        }],
        vec![AttentionHistoryEvent::NaggingStarted],
    );

    assert_eq!(view.items.len(), 2);
    assert!(matches!(view.items[0], HistoryItem::FocusCycle { .. }));
    assert_eq!(view.items[1], HistoryItem::NaggingStarted);
}
