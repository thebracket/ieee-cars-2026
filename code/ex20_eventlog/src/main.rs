// Lab: parse a JSON event log safely, and generate alert counts.
//
// Run from anywhere with: cargo run -p ex20_eventlog

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level {
    Info,
    Warn,
    Error,
}

#[allow(dead_code)] // `timestamp` and `message` are used by the lab exercises
#[derive(Debug, Deserialize)]
struct Event {
    timestamp: String,
    level: Level,
    source_ip: String,
    message: String,
}

#[derive(Debug, Error)]
enum LogError {
    #[error("could not read the log: {0}")]
    Read(#[from] std::io::Error),
    #[error("could not parse the log: {0}")]
    Parse(#[from] serde_json::Error),
}

fn load_events(path: &PathBuf) -> Result<Vec<Event>, LogError> {
    let raw = std::fs::read_to_string(path)?; // io::Error -> LogError::Read
    let events = serde_json::from_str(&raw)?; // serde_json::Error -> LogError::Parse
    Ok(events)
}

fn main() -> Result<(), LogError> {
    // Compile-time path so the sample works no matter where you run from:
    let log_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("events.json");
    let events = load_events(&log_path)?;

    let mut errors = 0;
    let mut warnings = 0;
    let mut errors_per_ip: HashMap<String, usize> = HashMap::new();

    for event in &events {
        match event.level {
            Level::Error => {
                errors += 1;
                *errors_per_ip.entry(event.source_ip.clone()).or_insert(0) += 1;
            }
            Level::Warn => warnings += 1,
            Level::Info => {}
        }
    }

    println!("Parsed {} events", events.len());
    println!("Errors: {errors}, Warnings: {warnings}");
    println!("Errors by source IP:");

    let mut counts: Vec<_> = errors_per_ip.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    for (ip, count) in counts {
        println!("  {ip}: {count}");
    }

    Ok(())
}
