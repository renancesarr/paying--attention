use std::{env, fs, io, path::PathBuf};

use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManualValidationOutcome {
    Passed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualValidationEvidence {
    pub ticket_number: u16,
    pub check_name: String,
    pub outcome: ManualValidationOutcome,
    pub observed_at: String,
    pub command: String,
    pub observation: String,
    pub artifact_path: Option<String>,
}

pub struct ManualValidationStore {
    connection: Connection,
}

pub fn manual_validation_database_path() -> io::Result<PathBuf> {
    let state_base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
        .ok_or_else(|| io::Error::other("XDG_STATE_HOME or HOME must be set."))?;
    let state_dir = state_base.join("paying-attention");
    fs::create_dir_all(&state_dir)?;
    Ok(state_dir.join("manual-validation.sqlite"))
}

impl ManualValidationStore {
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        Self::initialize(Connection::open_in_memory()?)
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> rusqlite::Result<Self> {
        Self::initialize(Connection::open(path)?)
    }

    pub fn record_manual_validation(
        &mut self,
        evidence: ManualValidationEvidence,
    ) -> rusqlite::Result<()> {
        self.connection.execute(
            "
            INSERT INTO manual_validation_evidence (
                ticket_number,
                check_name,
                outcome,
                observed_at,
                command,
                observation,
                artifact_path
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            params![
                evidence.ticket_number,
                evidence.check_name,
                outcome_name(evidence.outcome),
                evidence.observed_at,
                evidence.command,
                evidence.observation,
                evidence.artifact_path,
            ],
        )?;
        Ok(())
    }

    pub fn manual_validation_evidence_for(
        &self,
        ticket_number: u16,
    ) -> rusqlite::Result<Vec<ManualValidationEvidence>> {
        let mut statement = self.connection.prepare(
            "
            SELECT check_name, outcome, observed_at, command, observation, artifact_path
            FROM manual_validation_evidence
            WHERE ticket_number = ?1
            ORDER BY id
            ",
        )?;
        let evidence = statement
            .query_map(params![ticket_number], |row| {
                Ok(ManualValidationEvidence {
                    ticket_number,
                    check_name: row.get(0)?,
                    outcome: outcome_from_name(row.get::<_, String>(1)?.as_str())?,
                    observed_at: row.get(2)?,
                    command: row.get(3)?,
                    observation: row.get(4)?,
                    artifact_path: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(evidence)
    }

    fn initialize(connection: Connection) -> rusqlite::Result<Self> {
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS manual_validation_evidence (
                id INTEGER PRIMARY KEY,
                ticket_number INTEGER NOT NULL,
                check_name TEXT NOT NULL,
                outcome TEXT NOT NULL,
                observed_at TEXT NOT NULL,
                command TEXT NOT NULL,
                observation TEXT NOT NULL,
                artifact_path TEXT
            );
            ",
        )?;
        Ok(Self { connection })
    }
}

fn outcome_name(outcome: ManualValidationOutcome) -> &'static str {
    match outcome {
        ManualValidationOutcome::Passed => "passed",
        ManualValidationOutcome::Failed => "failed",
        ManualValidationOutcome::Blocked => "blocked",
    }
}

fn outcome_from_name(name: &str) -> rusqlite::Result<ManualValidationOutcome> {
    match name {
        "passed" => Ok(ManualValidationOutcome::Passed),
        "failed" => Ok(ManualValidationOutcome::Failed),
        "blocked" => Ok(ManualValidationOutcome::Blocked),
        _ => Err(rusqlite::Error::InvalidQuery),
    }
}
