# Capstone Walkthrough

Every piece of this is something we've covered today. Here's the thought process,
step by step. The finished code is `code/capstone_solution/src/main.rs`.

## Step 1: The Types — Already Done For You

```rust
#[derive(Debug, Deserialize)]
struct Flow {
    source_ip: String,
    destination_port: u16,
    bytes: u64,
}
```

Three fields per flow; the `Deserialize` derive is exactly what we did in the
event-log lab. The error enum is the same pattern too:

```rust
#[derive(Debug, Error)]
enum ReportError {
    #[error("could not read the traffic log: {0}")]
    Read(#[from] std::io::Error),
    #[error("could not parse the traffic log: {0}")]
    Parse(#[from] serde_json::Error),
}
```

And `load_flows` — read the file, parse it, `?` converts either failure into
`ReportError`, and `main` returns `Result` so both are reported cleanly:

```rust
fn load_flows(path: &PathBuf) -> Result<Vec<Flow>, ReportError> {
    let raw = std::fs::read_to_string(path)?;
    let flows = serde_json::from_str(&raw)?;
    Ok(flows)
}
```

## Step 2: The Totals

```rust
// The sample totals fit in u64; untrusted input needs checked_add.
let total_bytes: u64 = flows.iter().map(|f| f.bytes).sum();
println!("Flows: {}", flows.len());
println!("Total bytes: {total_bytes}");
```

`map` turns each flow into its byte count, `sum` adds them up. Note the
annotation: the compiler needs to know which integer type to sum into. The
addition is unchecked: fine for this fixed sample, but a crafted log could
overflow the total, so untrusted input calls for `checked_add` and a typed
error instead.

## Step 3: Bytes per Source IP

This is the `entry(...).or_insert(0) += value` counting pattern from the lab,
except we're *adding* bytes instead of counting events:

```rust
let mut bytes_per_ip: HashMap<String, u64> = HashMap::new();
for flow in &flows {
    *bytes_per_ip.entry(flow.source_ip.clone()).or_insert(0) += flow.bytes;
}
```

> Why `clone()`? `entry` needs to own its key, and `flow` only borrows it. It's
> a `String` clone per flow, which is fine at this scale — and being explicit
> about the cost is very Rust.

## Step 4: Sort and Find the Top Talker

`HashMap` iteration order is random, so collect it into a `Vec` and sort —
descending by bytes, so the top talker comes first:

```rust
let mut by_bytes: Vec<(String, u64)> = bytes_per_ip.into_iter().collect();
by_bytes.sort_by(|a, b| b.1.cmp(&a.1)); // swap a/b for largest-first

println!("Bytes by source IP:");
for (ip, bytes) in &by_bytes {
    println!("  {ip}: {bytes}");
}
if let Some((ip, bytes)) = by_bytes.first() {
    println!("Top talker: {ip} ({bytes} bytes)");
}
```

`b.1.cmp(&a.1)` is "compare bytes, biggest first". `first()` returns an
`Option` — an empty log has no top talker, and the program says so instead of
crashing. (The `if let` is the same one you used in `ex18_collections`.)

## Step 5: Flows per Destination Port

Same accumulation, keyed by port number this time — counting instead of summing:

```rust
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
```

`u16` ports are `Copy`, so there's no `clone()` needed — the key just copies
into the map.

## The Whole Thing

Put together, running `cargo run -p capstone_solution` prints:

```
Flows: 12
Total bytes: 81850
Bytes by source IP:
  203.0.113.99: 50000
  10.0.0.14: 23000
  192.168.4.7: 8000
  10.0.0.31: 850
Top talker: 203.0.113.99 (50000 bytes)
Flows by destination port:
  port 22: 3
  port 80: 2
  port 443: 5
  port 8080: 2
```

Everything used here — `Deserialize`, `HashMap` and the `entry` API, iterator
chains, `Option`/`Result`, `?`, `thiserror`, `sort` — appeared earlier today.
That's the toolkit for secure systems work: a log goes in, a typed analysis
comes out, and malformed JSON is reported as a typed `Err` instead of an
exception or a silent partial result.

> Notice the story the report tells: `203.0.113.99` sent the most bytes while
> making only three connections — and in the event-log lab earlier, that same
> address was the one failing authentication, repeatedly. In the real world
> those would be two different systems, and an analyst's job is joining the
> dots. Data out, meaning in: that's analysis.
