use std::{env, process};

use paying_attention_cli::TechnicalRecoveryService;
use paying_attention_storage::{application_database_path, SqliteAttentionStore};

fn main() {
    if let Err(message) = run(env::args().skip(1).collect()) {
        eprintln!("{message}");
        process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
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
        _ => Err("Use `paying-attention status` or `paying-attention unlock --force`.".into()),
    }
}
