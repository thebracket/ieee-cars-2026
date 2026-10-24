# Project Structure and Visibility

Some terminology first — it's looser in most languages, but precise in Rust:

* A **crate** is a Rust compilation unit: one library or one binary.
* A **binary** is a crate with `src/main.rs`. A **library** is a crate with
  `src/lib.rs`. A single `Cargo.toml` can describe a library crate and several
  binary crates.
* A **module** is a subdivision *within* a crate — Rust's unit of code
  organization and privacy.
* A **workspace** is several crates sharing one build: one `target/`, one
  lockfile, one `cargo build` for everything.

> The workshop's `code/` directory is itself a workspace — that's why
> `cargo check -p ex03_errors` worked, and why four example crates share one
> `target/` instead of quadrupling your disk. Every `exNN` crate is a member,
> listed in the root `Cargo.toml`.

> This is unlike C++, where `#include` is basically cut-and-paste and the C++20
> modules story varies by compiler. Rust's system is built in, and `cargo`
> understands it natively.

## Modules: Three Ways

The code for this is in `code/ex04_org`. Rust gives you three ways to make a
module — same visibility rules in all of them:

**1. Inline** — a namespace in the file:

```rust
pub mod greeting {
    pub fn hello() {
        println!("Hello from an inline module!");
    }
}
```

**2. File-based** — `mod config;` in `main.rs` pulls in `src/config.rs`.
The *file* is the namespace:

```rust
// src/config.rs -- reachable as `config::`
pub fn load(path: &str) -> String { /* ... */ }
```

**3. Directory** — `mod net;` pulls in `src/net/mod.rs`, and `net/` can grow
into a subtree of its own. This is how real projects scale from one file to
one hundred.

`mod` declarations form a tree rooted at `main.rs` (or `lib.rs`). A file
existing on disk does *nothing* until some `mod` line declares it — no
preprocessor magic, no "which of these 40 headers is actually included?"

## Private by Default

Everything is private unless you mark it `pub` — functions, structs, fields,
modules, crates. And here's the part that trips up C++ people: `pub` on a
module exposes *the module*, and each item inside still needs its own `pub`:

```rust
pub mod config {
    pub fn load() { /* ... */ }
    fn validate() { /* ... */ } // private: invisible outside
}
```

`ex04_org` has a private `validate` function in `config.rs`. Uncomment the last
line of `main` and try to call it:

```
error[E0603]: function `validate` is private
  --> src/main.rs:28:13
   |
28 |     config::validate("nope");
   |             ^^^^^^^^ private function
   |
note: the function `validate` is defined here
  --> src/config.rs:15:1
   |
15 | fn validate(text: &str) -> bool {
```

That's the *entire* access-control system. No `friend`, no headers-vs-impl
split, no `#define private public` hacks: the visibility boundary is the module,
enforced by the compiler. Internal helpers stay internal — which, if you're
writing anything security-adjacent, is exactly what you want the default to be.

> You'll also meet `pub use`, which re-exports an item under a different path —
> handy for flattening a deep module tree into a clean public API. Library
> authors use it constantly; you'll see it in Session 3 when we build a
> Python-facing API.
