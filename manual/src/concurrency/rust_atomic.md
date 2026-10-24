# Rust Atomic

> The code for this is in `code/ex11_rust_atomic`.


I'll admit that the C++ API is a little nicer to use here, but detecting that you needed an atomic and forgot is worth the price!

You can use an atomic pretty easily in Rust:

```rust
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let counter: AtomicUsize = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for i in 0..3 {
            scope.spawn(|| {
                for _i in 0..100_000 {
                    counter.fetch_add(1, Relaxed);
                }
            });
        }
    });
    println!("{}", counter.load(Relaxed));
}
```

## What's up the `Relaxed`?

Humorously, the Rust standard library documentation says "Refer to the C++ standard". So you may know better than I do. Atomics have ordering guarantees. `Relaxed` is the weakest guarantee, not promising that operations happen in any particular order or retaining memory barriers beyond making sure the operation completes. There's a bunch more. 

Mara Bos' book "Rust Atomics and Locks" ( https://marabos.nl/atomics/ ) is a great read for how this actually works.

## Why isn't `counter` mutable?

The `mut` keyword is about mutual exclusion more than mutability. Rust supports
a pattern called "interior mutability". You'll see this pattern again today---the
Mutex page coming up uses exactly the same trick.

For now: think of `mut` like a compile-time lock. If a type can provide that lock at run-time, it can opt out of the compile-time lock.