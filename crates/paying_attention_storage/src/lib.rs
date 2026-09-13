//! SQLite adapter for local Paying Attention product data.

use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttentionHistoryEvent {
    NaggingStarted,
    DriftRecoverySubmitted,
    MeetingModeStarted,
    TelegramFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusCycleRecord {
    pub declared_task: String,
    pub relevance: Option<String>,
    pub completion: Option<String>,
    pub completion_justification: Option<String>,
}

pub struct SqliteAttentionStore {
    connection: Connection,
}

impl SqliteAttentionStore {
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let connection = Connection::open_in_memory()?;
        Self::initialize(connection)
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> rusqlite::Result<Self> {
        Self::initialize(Connection::open(path)?)
    }

    fn initialize(connection: Connection) -> rusqlite::Result<Self> {
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS attention_history (
                id INTEGER PRIMARY KEY,
                event_type TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS technical_logs (
                id INTEGER PRIMARY KEY,
                message TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS restorable_state (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                snapshot TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS focus_cycles (
                id INTEGER PRIMARY KEY,
                declared_task TEXT NOT NULL,
                relevance TEXT,
                completion TEXT,
                completion_justification TEXT
            );
            ",
        )?;

        Ok(Self { connection })
    }

    pub fn record_history(&mut self, event: AttentionHistoryEvent) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO attention_history (event_type) VALUES (?1)",
            params![history_event_name(event)],
        )?;
        Ok(())
    }

    pub fn record_technical_log(&mut self, message: &str) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO technical_logs (message) VALUES (?1)",
            params![message],
        )?;
        Ok(())
    }

    pub fn history(&self) -> rusqlite::Result<Vec<AttentionHistoryEvent>> {
        let mut statement = self
            .connection
            .prepare("SELECT event_type FROM attention_history ORDER BY id")?;
        let events = statement
            .query_map([], |row| {
                history_event_from_name(row.get::<_, String>(0)?.as_str())
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(events)
    }

    pub fn technical_logs(&self) -> rusqlite::Result<Vec<String>> {
        let mut statement = self
            .connection
            .prepare("SELECT message FROM technical_logs ORDER BY id")?;
        let messages = statement
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(messages)
    }

    pub fn save_restorable_state(&mut self, snapshot: &str) -> rusqlite::Result<()> {
        self.connection.execute(
            "
            INSERT INTO restorable_state (singleton, snapshot) VALUES (1, ?1)
            ON CONFLICT(singleton) DO UPDATE SET snapshot = excluded.snapshot
            ",
            params![snapshot],
        )?;
        Ok(())
    }

    pub fn load_restorable_state(&self) -> rusqlite::Result<Option<String>> {
        self.connection
            .query_row(
                "SELECT snapshot FROM restorable_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .optional()
    }

    pub fn clear_restorable_state(&mut self) -> rusqlite::Result<()> {
        self.connection
            .execute("DELETE FROM restorable_state", [])?;
        Ok(())
    }

    pub fn record_focus_cycle(&mut self, record: FocusCycleRecord) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO focus_cycles (declared_task, relevance, completion, completion_justification) VALUES (?1, ?2, ?3, ?4)",
            params![record.declared_task, record.relevance, record.completion, record.completion_justification],
        )?;
        Ok(())
    }

    pub fn focus_cycles(&self) -> rusqlite::Result<Vec<FocusCycleRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT declared_task, relevance, completion, completion_justification FROM focus_cycles ORDER BY id",
        )?;
        let records = statement
            .query_map([], |row| {
                Ok(FocusCycleRecord {
                    declared_task: row.get(0)?,
                    relevance: row.get(1)?,
                    completion: row.get(2)?,
                    completion_justification: row.get(3)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(records)
    }
}

fn history_event_name(event: AttentionHistoryEvent) -> &'static str {
    match event {
        AttentionHistoryEvent::NaggingStarted => "nagging_started",
        AttentionHistoryEvent::DriftRecoverySubmitted => "drift_recovery_submitted",
        AttentionHistoryEvent::MeetingModeStarted => "meeting_mode_started",
        AttentionHistoryEvent::TelegramFailed => "telegram_failed",
    }
}

fn history_event_from_name(event_name: &str) -> rusqlite::Result<AttentionHistoryEvent> {
    match event_name {
        "nagging_started" => Ok(AttentionHistoryEvent::NaggingStarted),
        "drift_recovery_submitted" => Ok(AttentionHistoryEvent::DriftRecoverySubmitted),
        "meeting_mode_started" => Ok(AttentionHistoryEvent::MeetingModeStarted),
        "telegram_failed" => Ok(AttentionHistoryEvent::TelegramFailed),
        _ => Err(rusqlite::Error::InvalidQuery),
    }
}
