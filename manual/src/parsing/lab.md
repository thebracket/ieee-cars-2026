# Lab: The JSON Event Log

Time to put the morning together. The scenario: a security event log — one JSON
object per event — and you need **alert counts per source IP**.

```json
{"timestamp": "2026-10-27T13:00:19Z", "level": "error",
 "source_ip": "203.0.113.99", "message": "Authentication failure for user root"}
```

Everything we've covered shows up in this lab: file reading and the `?` operator,
enums, `Result` with a `thiserror` type, `HashMap`, iterators, and — today's topic —
safe parsing of untrusted text.

> The code for this is in `code/ex20_eventlog`. Run it:

```bash
cargo run -p ex20_eventlog
```

## Making the Data Deserializable

Serde's `Deserialize` derive maps JSON straight onto types. The `level` field
becomes an enum — invalid levels are a *parse error*, not a silently-accepted
string:

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level { Info, Warn, Error }

#[derive(Debug, Deserialize)]
struct Event {
    timestamp: String,
    level: Level,
    source_ip: String,
    message: String,
}
```

> `rename_all = "lowercase"` matches JSON's `"error"` to `Level::Error`. Without
> it, Serde expects the variant written exactly as it appears in Rust.

## Parsing Safely

Both failure modes — file missing, JSON malformed — are ordinary `Result` values,
converted into our error enum by `#[from]`:

```rust
#[derive(Debug, Error)]
enum LogError {
    #[error("could not read the log: {0}")]
    Read(#[from] std::io::Error),
    #[error("could not parse the log: {0}")]
    Parse(#[from] serde_json::Error),
}
```

And the whole program is fallible — `main` returns a `Result`; if anything goes
wrong, the program exits with a typed error rather than a crash:

```rust
fn main() -> Result<(), LogError> {
    // ...
    let events = load_events(&log_path)?;
    // ...
    Ok(())
}
```

## Counting Alerts

```rust
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
```

That `entry(...).or_insert(0)` chain is the idiom for "count things in a map";
give it a read. The output:

```
Parsed 14 events
Errors: 5, Warnings: 4
Errors by source IP:
  203.0.113.99: 4
  10.0.0.14: 1
```

One IP produced four authentication failures. *That's* the alert.

## Your Turn

1. Corrupt `events.json` (delete a brace) and run again — you'll get a typed
   `Parse` error naming the exact line and column of the problem. No crash, no
   undefined behavior: untrusted input can only ever produce an `Err`.
2. Count **warnings** per IP as well as errors.
3. Print only events after a given timestamp.
4. Stretch: take the log path from the command line — `std::env::args().nth(1)`
   is an `Option` (you've met it before).

**Next:** Fearless Concurrency. We're coming back to this log — and scanning it
in parallel.
