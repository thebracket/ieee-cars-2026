// Capstone starter: read traffic.json and print a traffic report.
//
// Run with: cargo run -p capstone_starter
//
// The types and the file loading are provided. Your job is in main():
// work through the TODOs. The full solution is in capstone_solution --
// peek if you get stuck (it doesn't count as cheating!).

// Starter skeleton: these warnings disappear as you fill in the TODOs.
#![allow(dead_code, unused_variables)]

use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct Flow {
    source_ip: String,
    destination_port: u16,
    bytes: u64,
}

#[derive(Debug, Error)]
enum ReportError {
    #[error("could not read the traffic log: {0}")]
    Read(#[from] std::io::Error),
    #[error("could not parse the traffic log: {0}")]
    Parse(#[from] serde_json::Error),
}

fn load_flows(path: &PathBuf) -> Result<Vec<Flow>, ReportError> {
    let raw = std::fs::read_to_string(path)?;
    let flows = serde_json::from_str(&raw)?;
    Ok(flows)
}

fn main() -> Result<(), ReportError> {
    let log_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("traffic.json");
    let flows = load_flows(&log_path)?;

    // TODO 1: Print the number of flows and the total bytes.
    // Hint: flows.len(), and flows.iter().map(|f| f.bytes).sum::<u64>()

    // TODO 2: Build a HashMap<String, u64> of bytes per source_ip.
    // Hint: the entry().or_insert(0) pattern from the event-log lab.

    // TODO 3: Sort it descending by bytes, print each entry, then the
    // "Top talker" line. (Vec<(String, u64)>, then .sort_by(...))

    // TODO 4: Build a HashMap<u16, usize> of flow counts per
    // destination_port, sort ascending by port, and print them.

    println!("TODO: your traffic report goes here");
    Ok(())
}
