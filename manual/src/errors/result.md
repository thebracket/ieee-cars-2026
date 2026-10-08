# Result: Errors You Can't Ignore

`Result<T, E>` is Option's louder sibling: a call that can *fail* returns
`Ok(value)` or `Err(error)` — never a half-written buffer and a global `errno`
you forgot to check.

Every language has a different answer here:

| Language | How errors travel | The catch |
|----------|-------------------|-----------|
| C | Return codes / NULL | Nothing forces a check; `errno` is a global |
| C++ | Exceptions (or C-style codes) | Uncaught = crash; throw is slow; `catch (...)` swallows |
| Python | Exceptions | Uncaught = traceback; some APIs quietly return `None` |
| Rust | A `Result<T, E>` enum | The compiler won't let you forget |

## The Compiler Is Watching

`ex03_errors` has a function like this:

```rust
fn load_config() -> Result<String, std::io::Error> {
    let text = std::fs::read_to_string(Path::new("config.json"))?;
    Ok(text)
}
```

Try just calling it:

```rust
load_config();
```

It compiles — with a warning:

```
warning: unused `Result` that must be used
 --> src/main.rs:19:5
  |
19 |     load_config();
   |     ^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
19 |     let _ = load_config();
   |     +++++++
```

The error still exists, the warning still fires. You *can* silence it:

```rust
let _ = load_config();
```

But now it's a code review-visible statement: *"I have decided to ignore this
error."* That's a completely different thing from C, where forgetting was free.

> This works via `#[must_use]` — an attribute you can put on your own types, too.

## The `?` Operator

Handling every error inline gets noisy fast. `?` propagates errors up the stack
while keeping the code looking like the happy path:

```rust
fn load_config() -> Result<String, std::io::Error> {
    let path = Path::new("config.json");
    let text = std::fs::read_to_string(path)?; // Err? Return it now. Ok? Unwrap it.
    Ok(text)
}
```

`?` means: if this is `Err`, return the error from *this function* immediately;
if it's `Ok`, hand me the contents. Same control flow as a page of `if (err)
return err;` in C — but one character, impossible to forget, and with no
exception overhead.

And because even `main` can return a `Result`, top-level errors become a clean
program exit with a message instead of a panic backtrace:

```rust
fn main() -> Result<(), std::io::Error> {
    let text = load_config()?;
    println!("{text}");
    Ok(())
}
```
