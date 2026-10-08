# Hello World, Called From Python

Now the traditional part. Open `src/lib.rs` and add a second function — *inside*
the `#[pymodule] mod ieee_cars_pyo3_demo { ... }` block:

```rust
    /// Say hello, from Rust.
    #[pyfunction]
    fn hello_rust() -> PyResult<()> {
        println!("Hello World from Rust");
        Ok(())
    }
```

`PyResult<()>` means "either `Ok(())` — success with nothing to return — or a
Python-sized error", which is the next page's topic. For now, note that the
function body is *ordinary Rust*.

## Build and Call It

```bash
maturin develop
```

```bash
python3
```

```
>>> import ieee_cars_pyo3_demo
>>> ieee_cars_pyo3_demo.sum_as_string(1, 2)
'3'
>>> ieee_cars_pyo3_demo.hello_rust()
Hello World from Rust
>>> quit()
```

That "Hello World" was printed by Rust, from a Python process, on the other side
of a language boundary. No C, no manual reference counting, no glue layer you
maintain by hand.

> `println!` is Rust's print — it writes to standard output, which is exactly the
> terminal Python is running in. Want to return the string to Python instead?
> Change the signature to `PyResult<String>` and `Ok("Hello World".to_string())`
> — and Python gets a `str` back. Type conversion is automatic.

The pattern generalizes: ordinary Rust functions in, typed Python callables out.
The interesting questions start when the function can *fail* — which brings us
to `PyResult` for real.
