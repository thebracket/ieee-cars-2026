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

So why can threads share an integer for reading, but not freely mutate it? Rust combines borrowing rules with two traits: `Send` and `Sync`.

* A `Send` type can have its ownership safely transferred to another thread.
* A `Sync` type can have shared references safely sent between threads: `T: Sync` means `&T: Send`.

`Send` and `Sync` are automatically implemented by the compiler when a type's components allow it. `Rc` is neither: its reference count is designed for single-threaded use.

A bare `i32` is both `Send` and `Sync`, without any synchronization primitive. Shared references prevent ordinary mutation of the value while those references are alive; the binding need not be `const` or permanently immutable.

Mutation through shared references requires interior mutability. Across threads, this needs an appropriate thread-safe mechanism, such as an atomic or a mutex. For example, `AtomicI32` supports shared reads with `load` and updates with operations such as `fetch_add`.

Rust provides atomics for all of the primitive types, but does *not* provide a general Atomic wrapper for other types. Rust's atomic primitives are pretty much a 1:1 match with CPU intrinsics, which don't generally offer sync+send atomic protection for complicated types.

## Mutexes

Atomics are for primitives. For complicated types, you need a `Mutex`---and Rust's Mutex *wraps* the data it protects, so it's physically impossible to touch the protected data without holding the lock. When the data has to outlive a single scope, you wrap the Mutex in an `Arc`---an atomically reference-counted shared pointer that you clone into each thread.

And here's the theme for the rest of this section: **C++ gives you the tools and trusts you to use them. Rust makes you pick them up.** Once we've covered the thread basics, we'll fix this exact race both ways---C++ first, then Rust, spot-the-difference style.

## Summary

Rust's data race protection combines borrowing rules with `Send` and `Sync`: ownership can cross threads when the type is `Send`, and shared references can cross threads when the type is `Sync`. These traits do not permit unrestricted shared mutation; that requires appropriate thread-safe interior mutability. Safe Rust prevents data races. `unsafe` does not turn off the borrow checker: it allows specific operations whose safety requirements the programmer must uphold.

All of these safety guarantees add up to create an environment in which common bugs are hard to create. You do have to jump through a few hoops, but once you are used to them---you can fearlessly write concurrent code knowing that Rust will make the majority of multi-threaded bugs a compilation error rather than a difficult debugging session.
