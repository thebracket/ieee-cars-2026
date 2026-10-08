# Why Doesn't this Work?

> The code for this is in `code/ex01_movefail` (yes, the compile error is intentional!).

Follow along. This will work in any sane language, right?

```rust
fn main() {
  let s = "Hello world".to_string();
  printme(s);
  println!("{s}"); // Like printf or Python formatting, println uses tags to express what to print
}

fn printme(s: String) {
  println!("{s}");
}
```

A nice, reasonable program. Unfortunately, `cargo run` gives an enormous error message!

```
   Compiling ex01_movefail v0.1.0 (/home/herbert/Documents/Ardan/ieee-cars-2026/code/ex01_movefail)
error[E0382]: borrow of moved value: `s`
 --> ex01_movefail/src/main.rs:4:16
  |
2 |     let s = "Hello, world".to_string();
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     printme(s);
  |             - value moved here
4 |     println!("{s}");
  |                ^ value borrowed here after move
  |
note: consider changing this parameter type in function `printme` to borrow instead if owning the value isn't necessary
 --> ex01_movefail/src/main.rs:7:15
  |
7 | fn printme(s: String) {
  |    -------    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
3 |     printme(s.clone());
  |              ++++++++
```

> I guess we're not in sane language territory anymore? That was my first reaction when I used Rust, at least.

## So What's Going On?

Rust is *move by default*. So what we actually wrote is:

```cpp
#include <iostream>
#include <string>
#include <utility>

void printme(std::string s) {
  std::cout << s << '\n';
}

int main() {
  std::string s = "hello world";
  printme(std::move(s));
  std::cout << s << '\n'; // Valid but unspecified behavior, s is "moved from". It varies by compiler/platform from an empty string, the original text, or a segfault!
}
```

C++ (and C) have the concept of *ownership*, but it isn't explicit. When you make a `unique_ptr`, you own it - and you *move* it to other owners. Rust makes this
explicit. Everything is owned by a scope.

This isn't just pedantry: *use-after-free* and *double-free* are two of the most
exploited bug classes in secure systems. CVE databases are full of them. Rust's
compiler refuses to build the program, so the bug class simply never ships.
