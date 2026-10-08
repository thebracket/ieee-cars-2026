# Ownership & Borrowing

Now that your toolchain is verified, let's build something real. Ownership is *the*
feature that makes Rust safe by default — and the sooner you meet it, the better.

1. Open a terminal.
2. `cd` to a parent directory you want to use.
3. Type `cargo new hello`
4. Type `cd hello`
5. Type `cargo run`

You should see "Hello, World". Congratulations - you have a working Rust toolchain!

## The Source

The directory contains:

- `src/main.rs` - the main file. If there's a main.rs, the "crate" is an executable. If it has `lib.rs`, it's a library. It can have both!
- `Cargo.toml` - the build manifest.
- `target` - object files go here.

The `main.rs` file looks like this:

```rust
fn main() {
  println!("Hello, World.");
}
```

So clearly `fn` is "function", and the entry point is `main`. Not dissimilar to:

```cpp
#include <iostream>

int main() {
  std::cout << "Hello, World.\n";
  return 0;
}
```

> I always say that "Hello World" is the worst possible introduction to Rust!

So why is `println` shouting? It's a *macro*. Rust doesn't support variadic functions (Rust 1.100 will make them a little easier in November!), so `println!` needs to be a macro. Unlike `#DEFINE`, macros have to be marked. So no `#define TRUE=LINE %2` for us!

In other words, don't try and take too much Rust learning from the `println!` syntax yet!
