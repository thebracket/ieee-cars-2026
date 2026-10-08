# Move it Around

> The code for this is in `code/ex02_movearound`.

So you *can* do this, passing ownership around:

```rust
fn printme(s: String) -> String {
  println!("{s}");
  s // Note that "return" isn't required, the last expression WITHOUT a semicolon is returned
}

fn main() {
  let s = "Hello world".to_string();
  let s = printme(s); // We're shadowing; technically, it's a whole new s!
  println!("{s}");
}
```

You do see this pattern in some functional programming, but it's not common. It is surprisingly performant, because
LLVM will almost always elide the actual copying in compiled code.

Normally, though - this really isn't how you'd structure your program.
