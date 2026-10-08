# Setup: venv, Maturin, and `maturin develop`

This is the page that makes the rest of the session possible. Follow along — the
TA team will circulate.

> **Venue wifi is a shared resource.** If you possibly can, do steps 1–2 before
> the session, and run one `maturin develop` to warm up the compile cache. The
> first build downloads and compiles PyO3 — much better on your hotel wifi than
> forty laptops sharing a ballroom's.

## 1. A Virtual Environment

We're using the demo project that's already in the repo:

```bash
cd pyo3/ieee_cars_pyo3_demo
python3 -m venv .venv
source .venv/bin/activate     # Windows: .venv\Scripts\activate
```

Your prompt grows a `.venv` prefix:

```
(.venv) herbert@bertix23:~/ieee-cars-2026/pyo3/ieee_cars_pyo3_demo$
```

> I won't stop you from shouting "Leeroy Jenkins" and installing into your root
> Python environment. Your operating system might.

## 2. Install Maturin

```bash
pip install maturin
```

## 3. How This Project Was Born (Anatomy)

The project itself was created with `maturin init`, choosing `pyo3` from the
binding menu. It makes exactly three files:

```
.
├── Cargo.toml
├── pyproject.toml
└── src
    └── lib.rs
```

`Cargo.toml` — a Rust *library* that builds as a C dynamic library:

```toml
[package]
name = "ieee_cars_pyo3_demo"
version = "0.1.0"
edition = "2024"

[lib]
name = "ieee_cars_pyo3_demo"
crate-type = ["cdylib"] # Build as a C dynamic library.

[dependencies]
pyo3 = "0.27.0"
rayon = "1.11.0"
```

`pyproject.toml` — the Python packaging metadata; `maturin` is the build backend:

```toml
[build-system]
requires = ["maturin>=1.8,<2.0"]
build-backend = "maturin"

[project]
name = "ieee_cars_pyo3_demo"
requires-python = ">=3.9"
dynamic = ["version"]
```

And `src/lib.rs` contains the `#[pymodule]` we met on the last page — including
the template's `sum_as_string`.

## 4. The One Command: `maturin develop`

```bash
maturin develop
```

This compiles the Rust, produces the shared library, and installs it into the
*active virtual environment* — so Python can import it. You'll see something
like:

```
📦 Built wheel for CPython 3.14 to /tmp/.../ieee_cars_pyo3_demo-0.1.0-....whl
✏️ Setting installed package as editable
🛠 Installed ieee_cars_pyo3_demo-0.1.0
```

> I keep typing `maturin build`, thinking "that's the cargo build of maturin".
> It isn't — `maturin build` produces a distributable wheel (useful later!),
> but it does *not* install into your venv. **`maturin develop`** is the
> edit-and-test loop. I have considered having it tattooed on my hand.

## 5. Try It in the REPL

```bash
python3
```

```
>>> import ieee_cars_pyo3_demo
>>> ieee_cars_pyo3_demo.sum_as_string(1, 2)
'3'
>>> quit()
```

A Rust function just ran inside Python, with automatic type conversion in both
directions. That's the whole trick — everything else is detail.

> **The loop you're going to live in:** edit `src/lib.rs` → `maturin develop` →
> `import` and try it in Python. Rust's compiler errors show up in your
> terminal; Python's exceptions show up in the REPL.
