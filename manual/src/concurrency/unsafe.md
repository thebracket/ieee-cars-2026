# "I Keep Hearing About Unsafe Rust..."

At some point in every Rust talk, a C programmer raises a hand: "Sure, but where's
the escape hatch? Sometimes I just need to do the thing."

There is one. It's called `unsafe`---and it's much narrower than you'd expect.

## What `unsafe` Actually Means

`unsafe` does **not** mean "this code is unsafe". It means: *the compiler could not
verify that this code upholds the safety rules---a human needs to check it.*

```rust
fn main() {
    let mut x = 5i32;
    let p = &mut x as *mut i32; // A raw pointer. Making one is safe!
    unsafe {
        *p += 1; // Dereferencing it: that's on you.
    }
    println!("{x}");
}
```

Creating a raw pointer is perfectly safe---it's just an address. *Dereferencing*
one is `unsafe`, because the compiler can no longer prove that the pointee is
alive, aligned, and not aliased. Writing `unsafe` is you signing a statement that
you checked.

> Think of `unsafe` as a review annotation, not a power-up. `grep -r unsafe`
> instantly lists every line of a Rust codebase where the compiler was asked to
> step aside. Your C codebase is *entirely* that---you just can't grep for it.

## Inside an `unsafe` Block, You Can...

...dereference raw pointers, call `extern "C"` functions (FFI), read or write
mutable statics, access union fields, and implement `unsafe` traits. That's
roughly the list---five abilities, each a classic route to memory corruption.

That's it. No arbitrary mischief. Everything else in the language stays fully
checked, even inside the block.

## The Four Levels of Unsafe

In practice, `unsafe` code falls into four categories:

* **JUST FINE**: Calling out to C. The C code was never compiler-checked anyway;
  Rust simply can't see across the boundary. Millions of programs do this daily.
* **WARNING**: Implementing something Rust can't verify, but a human *can*
  review---and then wrapping it in a safe API. Rust's own `Vec` and `LinkedList`
  do this internally: hairy `unsafe` inside, bulletproof API outside.
* **BE CAREFUL**: Skipping checks for speed. `get_unchecked` ditches bounds
  checking---but benchmark first! LLVM is alarmingly good at proving bounds
  checks unnecessary in release builds. Measure, *then* reach for it.
* **OH NO**: `unsafe` to "fix" a design problem. Hey, watch this. Great for
  conference demos; a really horrible idea for production code.

## The Escape Hatch Is Shrinking

Here's the part C folks should appreciate. For years, the classic way to "turn
Rust off" was `static mut`: a mutable global you could poke from anywhere inside
an `unsafe` block, recreating any data race you liked.

Recent editions have been closing even that. Referencing a mutable static now
draws a `static_mut_refs` warning---headed for a hard error---and the sanctioned
replacement is a raw pointer, so the unsafety lives visibly at the dereference.
Every loophole gets treated as a bug to fix, not a feature to keep. `unsafe`
isn't the floor falling out of the guarantees; it's a small, marked,
actively-patrolled door in the wall.

## For Your Codebase

* Where you can, **forbid it outright**: `#![forbid(unsafe_code)]` at the crate
  root turns any `unsafe` into a compile error. Most pure-logic crates can and
  should do this.
* Where you must (FFI, crypto primitives, OS interfaces), **contain it**: small,
  reviewed modules behind safe APIs---the WARNING pattern above.
* Audit it like the security-relevant code it is: your CVEs will live where the
  `unsafe` is, so make sure that's where the review time goes, too.

> Coming from C: safe Rust is what you wish your codebase were. `unsafe` Rust is
> just honest about exactly which lines aren't.

Now---enough hand-managing locks. Let me show you the lazy way: [Rayon](./rayon.md).
