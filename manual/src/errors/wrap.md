# Wrap Up: Custom Errors & Safe Math

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

A runnable version is in `ex03_errors`:

```bash
cargo run --bin custom_error -p ex03_errors
```

> The project's `Cargo.toml` already lists `thiserror` — how dependencies get
> there is exactly what the next section is about.

Now callers can `match` on `ConfigError::NotFound` *by name*. That's the payoff:
errors that are part of your API's type signature, not a comments-and-hope system.

## Safe Arithmetic

One last "can't ignore it". This is perfectly legal C — and undefined behavior:

```c
char a = 127;
a = a + 1; // Overflow: -128 in practice, UB in theory
```

In Rust, overflow **panics in debug builds** and wraps silently in release. That
difference is a footgun, so for anything security-relevant, be explicit — the
same theme as everything today:

```rust
let a: i8 = 127;
a.checked_add(1)    // Option<i32-ish>: None if it would overflow
a.saturating_add(1) // Clamps at the maximum: 127
a.wrapping_add(1)   // -128, opt-in wrapping (crypto code loves this)
```

And notice how it comes full circle: `checked_add` returns an `Option`. No
overflow flag to forget to check — the failure is *in the return type*.

## Mini-Exercise

Five lines, both of today's types. Write a program that:

1. Reads the first command-line argument — `std::env::args().nth(1)` — an `Option`!
2. Parses it as a number — `arg.parse::<i32>()` — a `Result`!
3. Divides 100 by it — `.checked_div(x)` — an `Option` again!
4. Prints the result, handling every failure case.

Next up: **Cargo and Dependencies** — how that `thiserror` line got into
`Cargo.toml`, and how to manage the supply chain without losing sleep.
