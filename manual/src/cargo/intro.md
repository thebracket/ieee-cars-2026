# Cargo and Dependencies

You've been using Cargo all morning, probably without noticing. `cargo new`,
`cargo run` — that's Cargo. It's Rust's build system, package manager, test
runner, documentation generator, and project scaffolder, in a single tool that
ships with the language.

> If you're coming from C++: think `cmake` + `make` + `conan`/`vcpkg` +
> `clang-format` + `doctest` + `doxygen` — one binary, zero config files before
> you need them. From Python: `pip` + `venv` + `pytest` + `black` + `sphinx`,
> with dependency resolution that actually backtracks instead of shrugging.

The commands you'll use constantly:

| Command | Purpose |
|---------|---------|
| `cargo new` / `cargo init` | Start a project / convert a directory to one |
| `cargo check` | Type-check *fast* — no codegen. Your inner loop. |
| `cargo build` / `cargo run` | Compile / compile-and-execute |
| `cargo test` | Run unit tests |
| `cargo fmt` | Format to the Rust style. There is one. Use it. |
| `cargo clippy` | The linter. It's good. Heed it. |
| `cargo doc --open` | Build docs from your source (and deps) |

> `cargo check` matters more than you'd think: the borrow checker runs during
> it, so you get full error feedback in a fraction of the build time. Keep a
> terminal running `cargo watch -x check` (if you install `cargo-watch`) and
> errors appear as you type.

## Anatomy of a Project

Every project this morning has had the same shape — let's actually read one.
From `ex03_errors`:

```toml
[package]
name = "ex03_errors"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror = "2"
```

* `[package]` — the metadata: name, version, and **edition**.
* `edition` — Rust has yearly editions; `2024` is current. Editions opt the
  compiler into new language defaults *without* breaking old code — a crate
  from 2015 still compiles.
* `[dependencies]` — that one line is all it took to give `ex03_errors` the
  `thiserror` derive macro. Where it came from is the rest of this section.

Also in the directory: `Cargo.lock` — the *exact* versions that were resolved,
last build. For programs (as opposed to libraries), commit it: it's what makes
"works on my machine" mean something.

> The code for this section is in `code/ex04_org`. Next: how projects are
> structured, and who's allowed to see what.
