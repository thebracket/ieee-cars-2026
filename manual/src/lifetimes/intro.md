# Lifetimes and Smart Pointers

## You've Been Using Lifetimes All Morning

A *lifetime* is just the region of code during which a reference is valid. You've
been relying on them since the ownership section:

* When the compiler refused to let you use `s` after a move, or drop `s` while a
  borrow was still alive — lifetime checks.
* When `thread::scope` let you borrow the parent's data, because the scope
  *guarantees* threads finish before the data dies — lifetimes.
* Every use-after-free we couldn't write in C — a lifetime violation, caught at
  compile time.

Most of the time you never type the word "lifetime". Rust *elides* (infers) them.

## When You Do Need Annotations

When a function takes several references and returns one, the compiler needs to
know which input the output borrows from:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`'a` here says: *the returned reference lives as long as the shorter of the two
inputs*. That's the shape of 90% of annotations you'll ever write — and the
compiler only asks when it genuinely can't work it out.

> `'static` is the special one: a reference valid for the whole program
> (`&'static str` literals). Seeing it in a signature means "this borrows from
> nothing that can be dropped".

## Gotcha: Self-References

One thing Rust won't let you do: a struct that holds a value *and* a reference to
that same value.

```rust
struct Broken {
    text: String,
    first_word: &str, // ERROR: borrows `text`, which lives inside `self`
}
```

If the struct moves, the reference dangles. Rust says no. The fixes (indices,
`Box`, `Pin`) are advanced — if you hit this as a beginner, use an offset or an
owned `String`.
