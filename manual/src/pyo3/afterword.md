# Afterword: Detaching for Threaded Python

**Optional further reading — not required for the live session.** We already
have a working extension and a measured speedup. There is always more to
explore: this chapter looks at long-running Rust calls from threaded Python.

## The GIL

A big Rust call from Python doesn't automatically make Python faster as a
*whole*. On a GIL-enabled CPython build, PyO3 holds the **Global Interpreter Lock**
for the duration of a call:
while Rust computes, other Python threads are blocked from executing bytecode.

For a long computation called from threaded Python, you can let go:

```rust
    /// The same analysis, but with the GIL released while Rust works.
    #[pyfunction]
    fn word_frequencies_detached(py: Python<'_>, text: &str) -> HashMap<String, usize> {
        let owned = text.to_string();
        py.detach(move || word_frequencies(&owned))
    }
```

The copy is a deliberate choice, not something `detach` requires. The closure
runs to completion before `detach` returns, so borrowing `text` directly also
compiles. Attached handles such as `Bound<'py, PyString>` cannot cross
this boundary. Owned handles such as `Py<PyString>` can cross it, but using
Python APIs on them requires attachment to the interpreter again. We copy here
so the code running without the GIL owns all of its data outright: a simple,
conservative habit for a teaching
demo. It is not a universal rule, and it does not make arbitrary Python objects
safe to share across threads.

The benchmark runs the analysis from four Python threads at once:

```
4 Python threads, each running the analysis (16 MB corpus):
  GIL held (default)                0.284s
  GIL released (detached)           0.129s
```

With the GIL held, the four calls serialize — you're watching a language boundary.
With `detach`, they overlap and the machine's cores stay busy. Same results, half
the wall-clock.

> **When should you reach for `detach`?** When the Rust work is long-running and
> called from a threaded Python program — servers are the classic case. For a
> quick function call, the GIL handoff costs more than it saves.

## Back to the Main Path

The [word stats demo](wordstats.md) measures the single-call speedup;
[packaging and wheels](packaging.md) completes the live session.
