use paying_attention_storage::{ManualValidationEvidence, ManualValidationOutcome};

pub fn render(ticket_number: u16, evidence: &[ManualValidationEvidence]) -> String {
    let mut report = format!(
        "# Manual Validation: Ticket {ticket_number}\n\n| Check | Outcome | Observed at | Command | Observation | Artifact |\n| --- | --- | --- | --- | --- | --- |\n"
    );
    for item in evidence {
        report.push_str(&format!(
            "| {} | {} | {} | `{}` | {} | {} |\n",
            table_cell(&item.check_name),
            outcome_label(&item.outcome),
            table_cell(&item.observed_at),
            table_cell(&item.command),
            table_cell(&item.observation),
            table_cell(item.artifact_path.as_deref().unwrap_or("")),
        ));
    }
    report
}

fn table_cell(value: &str) -> String {
    value.replace(['\r', '\n'], " ").replace('|', "\\|")
}

fn outcome_label(outcome: &ManualValidationOutcome) -> &'static str {
    match outcome {
        ManualValidationOutcome::Passed => "passed",
        ManualValidationOutcome::Failed => "failed",
        ManualValidationOutcome::Blocked => "blocked",
    }
}
