use std::{env, process};

use paying_attention_cli::{manual_validation_command, TechnicalRecoveryService};
use paying_attention_storage::{
    application_database_path, manual_validation_database_path, SqliteAttentionStore,
};

fn main() {
    if let Err(message) = run(env::args().skip(1).collect()) {
        eprintln!("{message}");
        process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    if let [command, remaining @ ..] = arguments.as_slice() {
        if command == "validation" {
            let database = manual_validation_database_path().map_err(|error| error.to_string())?;
            println!(
                "{}",
                manual_validation_command::execute(remaining, &database)?
            );
            return Ok(());
        }
    }

    let path = application_database_path().map_err(|error| error.to_string())?;
    let mut store = SqliteAttentionStore::open(path).map_err(|error| error.to_string())?;
    let mut service = TechnicalRecoveryService::new(&mut store);

    match arguments.as_slice() {
        [command] if command == "status" => {
            println!("{}", service.status().map_err(|error| error.to_string())?);
            Ok(())
        }
        [command, flag] if command == "unlock" && flag == "--force" => {
            service.force_unlock().map_err(|error| error.to_string())?;
            println!("unlocked");
            Ok(())
        }
        _ => Err("Use `paying-attention status`, `paying-attention unlock --force`, or `paying-attention validation`.".into()),
    }
}
