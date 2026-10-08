# Wrap Up

The toolkit is complete:

* **Lifetimes** — the validity regions of borrows. Inferred unless a function's
  output could come from more than one input; `'a` annotations tie them together.
* **Self-references** — a struct can't hold a reference to its own field. (Coming
  from C++: that was always a bug waiting to happen, you were just allowed to
  write it.)
* **RAII and `Drop`** — values clean up after themselves, exactly once, wherever
  they die. No leaks, no `goto cleanup`, no forgotten `free`.
* **Smart pointers** — `Box` (unique), `Rc` (shared, single-threaded), `Arc`
  (shared, thread-safe).

You now have every ingredient for the capstone: ownership, errors, threads,
locks, and the smart pointers that hold it all together.

Time to build something: **the file integrity checker**.
