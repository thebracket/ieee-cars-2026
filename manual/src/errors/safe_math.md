# Safe Math

> The code for this is in `code/ex03_errors_part3`.

One last "can't ignore it". This compiles cleanly in C — and is undefined
behavior:

```c
#include <limits.h>

int a = INT_MAX; // 2147483647 on typical platforms
a = a + 1;       // Signed overflow: undefined behavior
```

The compiler is allowed to assume signed overflow never happens; in practice it
often wraps to `INT_MIN`, which is exactly why the bug survives testing. (The
older `char` version of this example was really showing *integer promotion*:
`a + 1` is computed as an `int` before being converted back to `char` — a
separate surprise, but not the overflow this section is about.)

In Rust, overflow **panics in debug builds** and wraps silently in release. That
difference is a footgun, so for anything security-relevant, be explicit — the
same theme as everything today:

```rust
let a: i8 = 127;
a.checked_add(1)    // Option<i8>: None if it would overflow
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

A complete version is in `ex03_errors_part3`:

```bash
cargo run -p ex03_errors_part3 -- 5
cargo run -p ex03_errors_part3 -- 0
cargo run -p ex03_errors_part3 -- nope
```

Next up: **Cargo and Dependencies** — how that `thiserror` line got into
`Cargo.toml`, and how to manage the supply chain without losing sleep.
