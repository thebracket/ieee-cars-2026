# Borrowing/References

C++ has references, which are basically syntax sugar to a pointer. So does Rust!

```rust
fn printme(s: &String) {
  println!("{s}");
}

fn main() {
  let s = "Hello world".to_string();
  printme(&s);
  println!("{s}");
}
```

Notice that we have `&` at both the *caller* and the *function signature*. Rust is pedantic like that, but it saves you from bugs. I hope I'm not the only one who has accidentally tanked
performance by changing a function signature and suddenly copying data around!

## How about Mutability?

Rust is `const` by default, too. Bjarne Stroustrup has quipped that Rust is C++ with all the defaults flipped, and he's not entirely wrong.

```rust
fn do_it(s: &mut String) {
    s.push_str(", and everyone in it!");
}

fn main() {
    let mut s = "Hello world".to_string();
    do_it(&mut s);
    println!("{s}");
}
```

Notice that we've had to use `mut` a lot:
* `let mut s` - declare `s` and make it mutable.
* `do_it(&mut s)` - declare that the borrow is mutable.
* `fn do_it(s: &mut String)` - the function requires a mutable borrow.

This ties back to the same type of bug. Unless you've been careful about `const` everywhere, in C++ you can't be sure that a function isn't going to change the parameters you gave it - and it won't protect you from someone quietly changing the function signature. So Rust makes it all explicit.

> There's a good argument that `mut` actually stands for "compile time mutex", but we'll worry about that later.
