//! SQLite adapter for local Paying Attention product data.

use std::{env, fs, io, path::PathBuf};

use paying_attention_config::{
    AppConfig, MeetingConfig, NaggingConfig, NaggingVisualStyle, TelegramConfig, TimerConfig,
    XdgPaths,
};
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
    pub started_at: String,
    pub ended_at: String,
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
                started_at TEXT NOT NULL,
                ended_at TEXT NOT NULL,
                relevance TEXT,
                completion TEXT,
                completion_justification TEXT
            );
            CREATE TABLE IF NOT EXISTS app_config (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                boot_delay_minutes INTEGER NOT NULL,
                focus_cycle_minutes INTEGER NOT NULL,
                attention_block_idle_minutes INTEGER NOT NULL,
                focus_cycle_idle_minutes INTEGER NOT NULL,
                custom_sound_path TEXT,
                telegram_bot_token TEXT,
                telegram_chat_id TEXT,
                meeting_default_duration_minutes INTEGER NOT NULL,
                allowed_meeting_durations_minutes TEXT NOT NULL
            );
            ",
        )?;
        let _ = connection.execute("ALTER TABLE focus_cycles ADD COLUMN started_at TEXT", []);
        let _ = connection.execute("ALTER TABLE focus_cycles ADD COLUMN ended_at TEXT", []);

        Ok(Self { connection })
    }

    pub fn record_history(&mut self, event: AttentionHistoryEvent) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO attention_history (event_type) VALUES (?1)",
            params![history_event_name(event)],
        )?;
        Ok(())
    }

    pub fn save_app_config(&mut self, config: &AppConfig) -> rusqlite::Result<()> {
        self.connection.execute(
            "
            INSERT INTO app_config (
                singleton,
                boot_delay_minutes,
                focus_cycle_minutes,
                attention_block_idle_minutes,
                focus_cycle_idle_minutes,
                custom_sound_path,
                telegram_bot_token,
                telegram_chat_id,
                meeting_default_duration_minutes,
                allowed_meeting_durations_minutes
            ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(singleton) DO UPDATE SET
                boot_delay_minutes = excluded.boot_delay_minutes,
                focus_cycle_minutes = excluded.focus_cycle_minutes,
                attention_block_idle_minutes = excluded.attention_block_idle_minutes,
                focus_cycle_idle_minutes = excluded.focus_cycle_idle_minutes,
                custom_sound_path = excluded.custom_sound_path,
                telegram_bot_token = excluded.telegram_bot_token,
                telegram_chat_id = excluded.telegram_chat_id,
                meeting_default_duration_minutes = excluded.meeting_default_duration_minutes,
                allowed_meeting_durations_minutes = excluded.allowed_meeting_durations_minutes
            ",
            params![
                config.timers.boot_delay_minutes,
                config.timers.focus_cycle_minutes,
                config.timers.attention_block_idle_minutes,
                config.timers.focus_cycle_idle_minutes,
                config
                    .nagging
                    .custom_sound_path
                    .as_ref()
                    .map(|path| path.to_string_lossy().to_string()),
                config.telegram.as_ref().map(|telegram| &telegram.bot_token),
                config.telegram.as_ref().map(|telegram| &telegram.chat_id),
                config.meeting.default_duration_minutes,
                meeting_durations_to_string(&config.meeting.allowed_durations_minutes),
            ],
        )?;
        Ok(())
    }

    pub fn load_app_config(&self) -> rusqlite::Result<AppConfig> {
        self.connection
            .query_row(
                "
                SELECT
                    boot_delay_minutes,
                    focus_cycle_minutes,
                    attention_block_idle_minutes,
                    focus_cycle_idle_minutes,
                    custom_sound_path,
                    telegram_bot_token,
                    telegram_chat_id,
                    meeting_default_duration_minutes,
                    allowed_meeting_durations_minutes
                FROM app_config
                WHERE singleton = 1
                ",
                [],
                |row| {
                    let bot_token = row.get::<_, Option<String>>(5)?;
                    let chat_id = row.get::<_, Option<String>>(6)?;
                    let telegram = match (bot_token, chat_id) {
                        (Some(bot_token), Some(chat_id)) => {
                            Some(TelegramConfig { bot_token, chat_id })
                        }
                        _ => None,
                    };

                    Ok(AppConfig {
                        timers: TimerConfig {
                            boot_delay_minutes: row.get(0)?,
                            focus_cycle_minutes: row.get(1)?,
                            attention_block_idle_minutes: row.get(2)?,
                            focus_cycle_idle_minutes: row.get(3)?,
                        },
                        nagging: NaggingConfig {
                            visual_style: NaggingVisualStyle::DarkWhitePulse,
                            custom_sound_path: row.get::<_, Option<String>>(4)?.map(PathBuf::from),
                        },
                        telegram,
                        meeting: MeetingConfig {
                            default_duration_minutes: row.get(7)?,
                            allowed_durations_minutes: meeting_durations_from_string(
                                &row.get::<_, String>(8)?,
                            )?,
                        },
                    })
                },
            )
            .optional()
            .map(|config| config.unwrap_or_default())
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
            "INSERT INTO focus_cycles (declared_task, started_at, ended_at, relevance, completion, completion_justification) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![record.declared_task, record.started_at, record.ended_at, record.relevance, record.completion, record.completion_justification],
        )?;
        Ok(())
    }

    pub fn focus_cycles(&self) -> rusqlite::Result<Vec<FocusCycleRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT declared_task, COALESCE(started_at, ''), COALESCE(ended_at, ''), relevance, completion, completion_justification FROM focus_cycles ORDER BY id",
        )?;
        let records = statement
            .query_map([], |row| {
                Ok(FocusCycleRecord {
                    declared_task: row.get(0)?,
                    started_at: row.get(1)?,
                    ended_at: row.get(2)?,
                    relevance: row.get(3)?,
                    completion: row.get(4)?,
                    completion_justification: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(records)
    }
}

pub fn application_database_path() -> io::Result<PathBuf> {
    let data_base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .ok_or_else(|| io::Error::other("XDG_DATA_HOME or HOME must be set."))?;
    let database_file = XdgPaths::from_data_base(data_base).database_file;
    let parent = database_file
        .parent()
        .ok_or_else(|| io::Error::other("application database path has no parent"))?;
    fs::create_dir_all(parent)?;
    Ok(database_file)
}

fn meeting_durations_to_string(durations: &[u16]) -> String {
    durations
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn meeting_durations_from_string(value: &str) -> rusqlite::Result<Vec<u16>> {
    value
        .split(',')
        .map(str::parse::<u16>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| rusqlite::Error::InvalidQuery)
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
