# Wrap Up: Custom Errors

> The code for this is in `code/ex03_errors_part2`.

## Errors Are Enums, So Roll Your Own

Since errors are just values, *your* error types are enums too — one variant per
failure mode, data in the variants where it helps. Hand-writing them means
implementing the `Display` and `Error` traits, which is tedious boilerplate, so
the community uses the `thiserror` crate to generate it:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("config file not found")]
    NotFound,
    #[error("config file is empty")]
    Empty,
    #[error("unable to read config file")]
    Io(#[from] std::io::Error), // `?` auto-converts io::Error via #[from]
}
```

A runnable version is in `ex03_errors_part2`:

```bash
cargo run -p ex03_errors_part2
```

Now callers can `match` on `ConfigError::NotFound` *by name*: errors are part of
your API's type signature instead of a comments-and-hope system.
