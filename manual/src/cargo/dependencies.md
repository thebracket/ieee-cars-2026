# Dependencies and the Supply Chain

Back to that one line in `ex03_errors`:

```toml
[dependencies]
thiserror = "2"
```

One line, and your program compiles with a code-generating library pulled from
[crates.io](https://crates.io), Rust's central package registry. (`docs.rs`
auto-publishes API documentation for everything on it — when an LLM isn't
handy, `cargo doc --open` builds docs for your *dependencies*, too.)

## Semver, and the Lockfile

The `"2"` is shorthand for `^2.0.0`: "any 2.x.y ≥ 2.0.0". Within a version
range, `cargo` resolves once and records the exact versions in `Cargo.lock`.
After that:

* Every rebuild uses exactly the locked versions — commit the lockfile and your
  build is reproducible.
* `cargo update` bumps you to the newest versions *within* your ranges — you
  opt in, deliberately.
* `cargo build --locked` (use it in CI) fails rather than touching the lockfile.

> One gotcha: versions below `1.0` get special treatment. `"0.2"` means
> `>=0.2.0, <0.3.0` — the minor version is the "breaking" slot until 1.0.

## The Part That Should Interest You: Trust

crates.io is to Rust what PyPI is to Python and npm is to JavaScript — which
means it inherits the same brutal truth: **every dependency is arbitrary code
that runs in your process, with your permissions.** Publishing is deliberately
low-friction (no curation queue), so the supply-chain threat model is real:

* **Typosquatting** — `thiserror` vs `this_eror`. Read what you type.
* **Compromised maintainers** — a legit crate publishes a poisoned 2.0.1.
* **Transitive dependencies** — you audit your deps; who audits their 200?
  `cargo tree` shows you the whole graph. Go look at it for a real project
  sometime. It's... a lot.

Rust doesn't solve this — nothing does — but the ecosystem gives you real tools:

| Tool | Job |
|------|-----|
| `cargo audit` | Check `Cargo.lock` against the RustSec advisory database |
| `cargo deny` | Enforce policies: licenses, banned crates, duplicate versions |
| `cargo tree` | Show the dependency graph — and *why* each dep is there |
| `cargo vendor` | Vendor all sources for air-gapped / reproducible builds |
| `--locked` | CI refuses builds that would change the lockfile |

## A Sensible Checklist

For secure-systems work, the habits that matter:

1. **Minimize dependencies.** The cheapest supply chain to secure is a short
   one. `std` is surprisingly capable.
2. **Prefer well-known, widely-used crates** — popular crates get scrutiny
   (and RustSec coverage) that a 40-download crate never will.
3. **Commit the lockfile, build with `--locked`** in CI.
4. **Run `cargo audit` in CI** — it's one command; a new advisory fails the
   build instead of lurking in prod.
5. **Review a crate like you'd review a colleague's PR** — the code is on
   docs.rs/source; "popular" is a prior, not an audit.

> This is also why Rust tends to have *fewer, bigger* dependencies than npm:
> async runtimes, serialization, HTTP — the big ones (`tokio`, `serde`) are
> effectively community infrastructure, widely audited and quick to patch.

And you're already practicing the workflow: `thiserror` went into `ex03_errors`
with one `cargo add` and a version range you can now read fluently.
