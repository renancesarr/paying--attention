use std::path::Path;

use paying_attention_storage::{
    ManualValidationEvidence, ManualValidationOutcome, ManualValidationStore,
};

use crate::manual_validation_report;

pub fn execute(arguments: &[String], database: &Path) -> Result<String, String> {
    match arguments {
        [command, remaining @ ..] if command == "record" => record(remaining, database),
        [command, remaining @ ..] if command == "report" => report(remaining, database),
        _ => Err(
            "Use `paying-attention validation record` or `paying-attention validation report`."
                .into(),
        ),
    }
}

fn record(arguments: &[String], database: &Path) -> Result<String, String> {
    let evidence = ManualValidationEvidence {
        ticket_number: required(arguments, "--ticket")?
            .parse()
            .map_err(|_| "--ticket must be a number.")?,
        check_name: required(arguments, "--check")?,
        outcome: parse_outcome(&required(arguments, "--outcome")?)?,
        observed_at: required(arguments, "--observed-at")?,
        command: required(arguments, "--command")?,
        observation: required(arguments, "--observation")?,
        artifact_path: optional(arguments, "--artifact"),
    };
    ManualValidationStore::open(database)
        .map_err(|error| error.to_string())?
        .record_manual_validation(evidence)
        .map_err(|error| error.to_string())?;
    Ok("recorded".into())
}

fn report(arguments: &[String], database: &Path) -> Result<String, String> {
    let ticket_number = required(arguments, "--ticket")?
        .parse()
        .map_err(|_| "--ticket must be a number.")?;
    let store = ManualValidationStore::open(database).map_err(|error| error.to_string())?;
    let evidence = store
        .manual_validation_evidence_for(ticket_number)
        .map_err(|error| error.to_string())?;
    Ok(manual_validation_report::render(ticket_number, &evidence))
}

fn required(arguments: &[String], flag: &str) -> Result<String, String> {
    optional(arguments, flag).ok_or_else(|| format!("{flag} is required."))
}

fn optional(arguments: &[String], flag: &str) -> Option<String> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}

fn parse_outcome(value: &str) -> Result<ManualValidationOutcome, String> {
    match value {
        "passed" => Ok(ManualValidationOutcome::Passed),
        "failed" => Ok(ManualValidationOutcome::Failed),
        "blocked" => Ok(ManualValidationOutcome::Blocked),
        _ => Err("--outcome must be passed, failed, or blocked.".into()),
    }
}
