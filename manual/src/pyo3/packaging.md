# Maturin: Packaging and Wheels

> The code for this is in `pyo3/ieee_cars_pyo3_demo`.

Your Python colleagues don't want your source directory — they want something
`pip install` can consume. That's a **wheel**: a zip file containing your
compiled extension, metadata, and hashes.

`maturin` builds one in a single command.

## Build It

```bash
cd pyo3/ieee_cars_pyo3_demo
source .venv/bin/activate
maturin build --release
```

```
🔗 Found pyo3 bindings
   Finished `release` profile [optimized] target(s) in ...
📦 Built wheel for CPython 3.14 to .../target/wheels/ieee_cars_pyo3_demo-0.1.0-cp314-cp314-manylinux_2_34_x86_64.whl
```

Note the difference from `maturin develop`: this does **not** install anything.
It produces a distributable artifact.

## Reading the Wheel Filename

```
ieee_cars_pyo3_demo - 0.1.0 - cp314 - cp314 - manylinux_2_34_x86_64 .whl
      name            version  python   abi      platform
```

* `cp314` — built for CPython 3.14 specifically. Wheels are usually
  **per-Python-version**; PyO3 can build `abi3` wheels that work across versions
  if you need broader reach.
* `manylinux_2_34` — built against glibc 2.34, so it runs on that or newer.
  This is the "Linux compatibility promise" encoded in the filename.
* `x86_64` — the CPU architecture.

Cross-platform builds for macOS/Windows/older Linux are what CI is for — the
`maturin-action` GitHub Action builds the full matrix for you.

## What's Inside

```bash
unzip -l target/wheels/ieee_cars_pyo3_demo-*.whl
```

```
  Length      Date    Time    Name
  159     ...  ieee_cars_pyo3_demo/__init__.py
792992     ...  ieee_cars_pyo3_demo/ieee_cars_pyo3_demo.cpython-314-x86_64-linux-gnu.so
  268     ...  ieee_cars_pyo3_demo-0.1.0.dist-info/METADATA
  109     ...  ieee_cars_pyo3_demo-0.1.0.dist-info/WHEEL
28836     ...  ieee_cars_pyo3_demo-0.1.0.dist-info/sboms/ieee_cars_pyo3_demo.cyclonedx.json
  593     ...  ieee_cars_pyo3_demo-0.1.0.dist-info/RECORD
```

The `.so` is your compiled Rust. `RECORD` holds hashes of every file, and if you
look closely: maturin ships a **software bill of materials** (SBOM) automatically —
a machine-readable list of what went into the build. In a secure-systems setting,
that's free supply-chain evidence; keep it.

> The version comes from your `Cargo.toml` (that's what `dynamic = ["version"]`
> in `pyproject.toml` means). One version number, one source of truth.

## Install and Use It Somewhere Else

Prove it's a real package — install the wheel into a *fresh* environment and run
it from an unrelated directory:

```bash
python3 -m venv /tmp/demo-venv
source /tmp/demo-venv/bin/activate
pip install /path/to/ieee_cars_pyo3_demo-0.1.0-cp314-cp314-manylinux_2_34_x86_64.whl
cd /tmp
python3
```

```python
>>> import ieee_cars_pyo3_demo as demo
>>> demo.sum_as_string(1, 2)
'3'
>>> demo.top_words("the fox the dog the", 2)
[('the', 3), ('dog', 1)]
```

No maturin, no Rust toolchain, no source — just a wheel and a Python. That's what
your colleagues receive.

## The Command Recap

| Command | What it does |
|---------|--------------|
| `maturin develop` | Compile *and install* into the active venv — the edit/test loop |
| `maturin develop --release` | The same, optimized — use it for benchmarks |
| `maturin build --release` | Produce a wheel in `target/wheels/` |
| `maturin publish` | Build and upload to PyPI (when you're ready for the world) |

> Secure-systems postscript: a wheel is a binary artifact, and `pip install`
> trusts it. Pin versions *and hashes* in your `requirements.txt`, review what
> you publish, and treat the SBOM as part of the deliverable.

## That's the Session

You've gone from a Python function to a compiled, parallel Rust
extension, packaged as a wheel and installed into a clean environment. The same
pattern scales from a word counter to packet parsers, crypto kernels, and data
engines — Python for the orchestration, Rust for the hot paths.

**Resources for the road:**

* The PyO3 guide — <https://pyo3.rs>
* Maturin — <https://www.maturin.rs>
* The Rust Book — <https://doc.rust-lang.org/book/>
* Rayon — <https://docs.rs/rayon>

The extension works, the release benchmark shows a meaningful speedup, and the
wheel makes it shareable. That's a complete result to take home. There is always
more: the [optional afterword](afterword.md) explores `detach` and threaded Python
at your own pace; it is not required for the live session.

Any questions? Let's hear them.
