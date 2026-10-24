// Capstone solution: read traffic.json and print a traffic report.
//
// Run with: cargo run -p capstone_solution

use serde::Deserialize;
use std::collections::HashMap;
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

    // Part 1: the basics.
    // The sample data fits comfortably in u64, so plain addition is fine here.
    // A real collector reading untrusted logs would use checked_add for this
    // total and the per-IP total below, and report an overflow error.
    let total_bytes: u64 = flows.iter().map(|f| f.bytes).sum();
    println!("Flows: {}", flows.len());
    println!("Total bytes: {total_bytes}");

    // Part 2: bytes per source IP.
    let mut bytes_per_ip: HashMap<String, u64> = HashMap::new();
    for flow in &flows {
        *bytes_per_ip.entry(flow.source_ip.clone()).or_insert(0) += flow.bytes;
    }
    let mut by_bytes: Vec<(String, u64)> = bytes_per_ip.into_iter().collect();
    by_bytes.sort_by(|a, b| b.1.cmp(&a.1)); // descending
    println!("Bytes by source IP:");
    for (ip, bytes) in &by_bytes {
        println!("  {ip}: {bytes}");
    }
    if let Some((ip, bytes)) = by_bytes.first() {
        println!("Top talker: {ip} ({bytes} bytes)");
    }

    // Part 3: flow counts per destination port.
    let mut flows_per_port: HashMap<u16, usize> = HashMap::new();
    for flow in &flows {
        *flows_per_port.entry(flow.destination_port).or_insert(0) += 1;
    }
    let mut by_port: Vec<(u16, usize)> = flows_per_port.into_iter().collect();
    by_port.sort(); // ascending by port
    println!("Flows by destination port:");
    for (port, count) in by_port {
        println!("  port {port}: {count}");
    }

    Ok(())
}
