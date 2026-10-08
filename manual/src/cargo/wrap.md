# Wrap Up: You Have a Cargo Project

That's the natural boundary for Session 1. This morning you went from
"Hello, World" to a real toolchain workflow:

* **Ownership and borrowing** — the compiler tracks every value and every
  reference, and use-after-free simply doesn't compile.
* **Safe error handling** — `Option` for "might not be there", `Result` for
  "might fail", `?` for propagation, and errors as ordinary enums you can
  design into your API.
* **Cargo** — one tool for building, testing, and depending; modules with
  private-by-default visibility; a lockfile and audit tooling for the supply
  chain.

You now have a Cargo project with dependencies. That's not a toy — that's the
starting point for every real Rust codebase.

## After Lunch

Session 2 puts it to work: parsing and data handling with **Serde** — which,
now that you can read `[dependencies]`, is the most-deployed dependency family
in the Rust ecosystem — then fearless concurrency, and the lifetimes you've
been using all day without knowing it. We'll finish with a capstone: a
parallel file-integrity checker.

> If you're staying for Session 3, make sure Python with `venv` support is
> installed — we'll bridge all of this into Python with PyO3.
