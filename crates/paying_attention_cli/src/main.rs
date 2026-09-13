use std::{env, fs, path::PathBuf, process};

use paying_attention_cli::TechnicalRecoveryService;
use paying_attention_storage::SqliteAttentionStore;

fn main() {
    if let Err(message) = run(env::args().skip(1).collect()) {
        eprintln!("{message}");
        process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    let path = state_database_path()?;
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

fn state_database_path() -> Result<PathBuf, String> {
    let state_base = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
        .ok_or_else(|| "XDG_STATE_HOME or HOME must be set.".to_string())?;
    let state_dir = state_base.join("paying-attention");
    fs::create_dir_all(&state_dir).map_err(|error| error.to_string())?;
    Ok(state_dir.join("state.sqlite"))
}
