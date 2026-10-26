# Option: No More Null

> The code for this is in `code/ex03_errors`.

Safe Rust has no null. Not "null but you have to be careful" — *no null*. When a
value may or may not be present, you get an `Option<T>`: `Some(value)` or `None`.
There is no third state, and there is no pretending an `Option<String>` is a
`String`.

```rust
let mut users = HashMap::new();
users.insert("herbert", 47);

// get() returns Option<&i32> -- Some(age) or None.
match users.get("root") {
    Some(age) => println!("root is {age}"),
    None => println!("No user named root"), // You MUST decide what happens
}
```

Try to use the value without handling `None`:

```rust
let age = users.get("root");
println!("{age}"); // Prints "Some(47)" or "None" -- not a dereference!
```

...and you'll find the compiler simply won't let you treat it as the inner type.
The closest you can get is `unwrap()`:

```rust
let age = users.get("root").unwrap(); // Panics if None
```

`unwrap()` says "I'm confident this is `Some` — crash otherwise". That's honest:
you've made the failure *explicit*, and it fails loudly in exactly one spot rather
than segfaulting three frames later with no clue where the null came from.

> `expect("message")` is `unwrap()` with a panic message attached. Use it in
> prototypes and tests; reach for `match`/`if let` in production paths.

## One-Variant Matching

Often you only care about the happy path. `if let` is a one-arm `match`:

```rust
if let Some(age) = users.get("herbert") {
    println!("herbert is {age}");
}
```

If the pattern doesn't match — the value was `None` — the block just doesn't run.

> Compare this with Python, where `users["root"]` raises `KeyError` at runtime, or
> `users.get("root")` quietly returns `None` and blows up three lines later. Rust
> moves that whole surprise to compile time.
