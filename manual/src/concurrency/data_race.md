# Data-Race Protection

> The code for this is in `code/ex05_data_race`.


Rust makes the bold claim that it offers "fearless concurrency" and no more data-races (within a program; it can't do much about remote calls). That's a very bold claim, and one I've found to be true so far---I'm much more likely to contemplate writing multi-threaded (and async) code in Rust now that I understand how it prevents me from shooting myself in the foot.

## An Example of a Data Race

Here's a little modern C++ program with a very obvious data-racing problem (paste it into [cpp.sh](https://cpp.sh) to try it):

```c++
#include <thread>
#include <iostream>

int main() {
    int counter = 0;
    std::thread t1([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    std::thread t2([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}
```

The program compiled and ran without any warnings (although additional static analysis programs would probably flag this).

The program fires up two threads. Each loops, incrementing a counter. It joins the threads, and prints the result. The predictable result is that every time I run it, I get a different result: 1015717, 1028094, 1062030 from my runs.

This happens because incrementing an integer isn't a single-step operation:

1. The CPU loads the current counter value, into a register.
2. The CPU increments the counter.
3. The CPU writes the counter back into memory.

There's no guaranty that the two threads won't perform these operations while the other thread is also doing part of the same operation. The result is data corruption.

Let's try the same thing in Rust. We'll use "scoped threads" (we'll cover threading properly in a few pages) to make life easier for ourselves. Don't worry about the semantics yet:

```rust
fn main() {
    let mut counter = 0;
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter += 1;
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter += 1;
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    println!("{counter}");
}
```

And now you see the beauty behind the "single mutabile access" rule: the borrow checker prevents the program from compiling, because the threads are mutably borrowing the shared variable. No data race here!

## Atomics

If you've used `std::thread`, you've probably also run into atomic types. An atomic operation is guaranteed to be completed in one CPU operation, and optionally be synchronized between cores. The fix for our racing counter is a one-liner in either language---make it `std::atomic_int` in C++, or `AtomicU32` with `fetch_add(1, Relaxed)` in Rust. We'll put the two versions side by side in a few pages, and see what each language makes you do to get there. The atomic version of this demo is in `code/ex05_data_race`.

## Why Does This Work?

So how does Rust know that it *isn't* safe to share an integer---but it is safe to share an atomic? Rust has two traits that self-implement (and can be overridden in unsafe code): `Sync` and `Send`.

* A `Sync` type can be safely used with a reference between threads - it has a *synchronization* primitive.
* A `Send` type can be sent between threads - it isn't going to do bizarre things because it is being accessed from multiple places.

Send and Sync are automatically implemented by the compiler when they apply. The majority of types are `Send` (`Rc` explicitly opts out of being `Send` - it is a *single-threaded* reference counter). Most types are not `Sync` - but synchronization primitives (atomics, mutexes, etc.) provide `Sync` safety:
* You can access an atomic or mutex across threads - while still allowing controlled mutation - safely because the synchronization primitive provides `Sync`.

For example, a bare `i32` can be sent between threads, but only safely referenced across threads if it is constant. An `AtomicI32` guarantees thread-safe mutation, so it can be safely read (via the `load` call) across threads.

Rust provides atomics for all of the primitive types, but does *not* provide a general Atomic wrapper for other types. Rust's atomic primitives are pretty much a 1:1 match with CPU intrinsics, which don't generally offer sync+send atomic protection for complicated types.

## Mutexes

Atomics are for primitives. For complicated types, you need a `Mutex`---and Rust's Mutex *wraps* the data it protects, so it's physically impossible to touch the protected data without holding the lock. When the data has to outlive a single scope, you wrap the Mutex in an `Arc`---an atomically reference-counted shared pointer that you clone into each thread.

And here's the theme for the rest of this section: **C++ gives you the tools and trusts you to use them. Rust makes you pick them up.** Once we've covered the thread basics, we'll fix this exact race both ways---C++ first, then Rust, spot-the-difference style.

## Summary

Rust's data race protection is very thorough. The borrow-checker prevents multiple mutable accesses to a variable, and the `Sync+Send` system ensures that variables that are accessed in a threaded context can both be sent between threads and safely mutated from multiple locations. It's extremely hard to create a data race in safe Rust (you can use the `unsafe` tag and turn off protections if you need to)---and if you succeed in making one, the Rust core team will file it as a bug.

All of these safety guarantees add up to create an environment in which common bugs are hard to create. You do have to jump through a few hoops, but once you are used to them---you can fearlessly write concurrent code knowing that Rust will make the majority of multi-threaded bugs a compilation error rather than a difficult debugging session.
