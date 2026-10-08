# RAII and Drop

C++ invented RAII — Resource Acquisition Is Initialization — and Rust adopts it
wholesale. There's no `~MyClass()`: destructors are implemented by a trait named
`Drop`.

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("I was dropped");
    }
}
```

Whenever a value leaves scope, `drop` runs before the memory is released. This is
why you haven't managed a single resource all day: `String`, `Vec`, `File`,
`MutexGuard`, `Box` — all of them clean up through `Drop`. No `goto cleanup`, no
forgotten `free`, no leak.

`Drop` fires *exactly once*, wherever the value actually dies — including in the
middle of a move:

> The code for this is in `code/ex13_drop` and `code/ex16_borrow_checker`.

```rust
fn do_it(a: MyStruct) { /* ... */ }          // owns a; drops it on return
fn move_it(a: MyStruct) -> MyStruct { a }    // ownership returned; no drop

fn main() {
    let a = MyStruct { s: "1".to_string() };
    do_it(a);                                // dropped inside do_it

    let b = MyStruct { s: "2".to_string() };
    let b = move_it(b);                      // still alive; dropped at end of main
    println!("{}", b.s);
}
```

Prints:

```
do_it called
Dropping: 1
move_it called
2
Dropping: 2
```

And when you *want* to clean up early, `std::mem::drop(value)` moves the value
into a function that drops it immediately — the standard way to release a lock or
close a file before the end of the enclosing scope.

> RAII is central to Rust's safety model, and we should credit C++ with its
> invention every time we mention it. (We do.)
