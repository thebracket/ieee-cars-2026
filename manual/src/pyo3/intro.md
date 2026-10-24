# Introducing PyO3

Python is the world's best glue language — until the glue becomes the bottleneck.
PyO3 ([github.com/PyO3/pyo3](https://github.com/PyO3/pyo3)) is the Rust library
that makes it easy to keep your Python and move the hot part into Rust.

PyO3 uses Rust's procedural macros to let you *declaratively* mark Rust modules as
Python modules and Rust functions as Python functions:

```rust
#[pyo3::pymodule]
mod string_sum {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
```

That's a complete, importable Python module — written in Rust.

## How Python Libraries Actually Work

When you type `import mymodule`, Python searches `sys.path` for either
`mymodule.py` or a compiled `mymodule.so` (`.dll` on Windows). Compiled modules
are loaded with `dlopen()`, and Python looks for one specific symbol:
`PyInit_mymodule` — which must match the module name.

The module is a C API citizen: `PyMODINIT_FUNC PyInit_mymodule(void)` returns a
`PyObject *`. You *can* write that by hand in C. You'd rather not:

* Manual reference counting — every path, every error, every early return.
* Type conversion by hand, between Python objects and C types.
* Error paths that must return `NULL` and set the exception — miss one and you
  get mysterious behaviour.

PyO3 generates that plumbing for you, and **maturin** packages the result as a
standard Python **wheel** — the thing `pip` installs. (We'll build one before the
session ends.)

## What We'll Do Today

1. **Set up a project** — venv, maturin, and the `maturin develop` loop.
2. **Hello World from Rust**, called from Python — the moment it clicks.
3. **Errors across the boundary** — `PyResult`, where Rust's `Err` becomes a
   Python exception.
4. **The word stats demo** — a real Rust function, parallelized with Rayon, with
   a measured release-build speedup.
5. **Package it** — `maturin build`, a wheel, and `pip install`.

> Everything lives in the repo's `pyo3/` directory — deliberately *outside* the
> Cargo workspace. Workspaces and maturin don't play nicely together, and this
> is the standard way to keep your Rust-for-Python projects separate.
