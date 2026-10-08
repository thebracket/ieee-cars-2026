# Environment Check

Before we start, let's verify that your toolchain is ready.

1. Open a terminal.
2. Type `rustup check` — this verifies both `rustup` and your installed toolchain are up-to-date.
3. Type `cargo --version` and `rustc --version` — any recent stable version is fine.

One-liner PASS/FAIL check:

```bash
cargo --version && rustc --version && rustup check && echo PASS || echo FAIL
```

If you see `PASS`, you're good to go. If not — raise a hand and one of the TAs will
come around and get you sorted.

## Editor

Any text editor works, but we recommend one with the Rust Analyzer language server:

* **VS Code** — install the `rust-analyzer` extension.
* **JetBrains IDEs** — the `Rust` plugin bundles it.
* **Helix / Neovim / Emacs** — `rust-analyzer` via your LSP setup.

Rust Analyzer gives you inline type information, compiler errors as you type, and
`cargo` integration. It turns "fighting the borrow checker" into "reading a helpful
explanation of the borrow checker".

> Stuck? Don't struggle in silence — the TAs are circulating for exactly this.
