# PyResult: Errors Across the Fence

Python handles errors with **exceptions**. Rust handles them with **`Result`** —
a value that's either `Ok(T)` or `Err(E)`, which you cannot accidentally ignore.

PyO3 bridges the two: when a Rust function returns `PyResult<T>`, an `Err` is
raised on the Python side as an exception, and an `Ok` is returned as a value.
The signature is just a type alias:

```rust
pub type PyResult<T> = Result<T, PyErr>;
```

Hmm, `Result`... an enum with two variants... a sum type... where have we seen
one of those today? Python gets its exception; Rust never grew exceptions.

## A Function That Can Fail

Add `divide` — again inside the module:

```rust
    /// Divide two numbers, raising a Python exception on failure.
    #[pyfunction]
    fn divide(a: i32, b: i32) -> PyResult<i32> {
        use pyo3::exceptions::PyArithmeticError;
        if b == 0 {
            Err(PyArithmeticError::new_err(
                "Dividing by zero will ruin your day",
            ))
        } else {
            Ok(a / b)
        }
    }
```

> Watch where the `use` lives: **inside** the module. An import at the crate
> root isn't visible to `divide`, because `divide` belongs to the nested
> `#[pymodule]` module. This one bites everyone once.

Build, then try both paths in the REPL:

```bash
maturin develop
```

```
>>> import ieee_cars_pyo3_demo
>>> ieee_cars_pyo3_demo.divide(10, 3)
3
>>> ieee_cars_pyo3_demo.divide(1, 0)
Traceback (most recent call last):
  ...
ArithmeticError: Dividing by zero will ruin your day
```

The `ArithmeticError` appeared as a normal Python exception. It's catchable with
`try`/`except` like any other — but on the Rust side it was just a value that
`divide` *had* to construct and return. The compiler made sure the error case
was handled; Python never has to know the difference.

## Why This Matters for Secure Systems

At every language boundary, there are two classic failure modes: an error is
silently swallowed, or it takes the process down. `PyResult` gives you neither:

* Rust can't forget to handle the failure — the type system won't allow it.
* Python gets a typed exception it can catch, log, or propagate.
* There are no exceptions in Rust to unwind through your data structures at
  exactly the wrong moment.

The standard exception types (`PyValueError`, `PyOSError`, `PyKeyError`...) live
in `pyo3::exceptions`, and your `Err` can carry any of them — or be constructed
from something richer later.

Next: put a *real* Rust function behind Python — the word stats demo, with Rayon
under the hood and the GIL to talk about.
