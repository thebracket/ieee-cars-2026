# Rust Mutex

> The code for this is in `code/ex12_rust_mutex` (the second, working listing).


Rust's Mutex is similar to C++'s: it provides mutual exclusion, and an RAII lock-guard system makes it harder to forget to unlock your mutex. There's a big difference, too: a Mutex *wraps* the data it is protecting.

```rust
use std::thread;
use std::sync::Mutex;

fn main() {
    let my_safe_data: Mutex<i32> = Mutex::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            for _i in 0..10000 {
                let mut lock = my_safe_data.lock().unwrap();
                *lock += 1;
            }
        });

        scope.spawn(|| {
            for _i in 0..10000 {
                // We can't get to the data without locking!
                //let mut lock = my_safe_data.lock().unwrap();
                my_safe_data += 1;
            }
        });
    });

    let lock = my_safe_data.lock().unwrap();
    println!("{}", *lock);
}
```

Commenting out the second `Mutex` won't work: you can't (reasonably) get to the protected data. So forgetting to lock your `Mutex` isn't possible.

Uncomment the `.lock()` call --- and it works as expected.

```rust
use std::thread;
use std::sync::Mutex;

fn main() {
    let my_safe_data: Mutex<i32> = Mutex::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            for _i in 0..10000 {
                let mut lock = my_safe_data.lock().unwrap();
                *lock += 1;
            }
        });

        scope.spawn(|| {
            for _i in 0..10000 {
                let mut lock = my_safe_data.lock().unwrap();
                *lock += 1;
            }
        });
    });

    let lock = my_safe_data.lock().unwrap();
    println!("{}", *lock);
}
```

> Why are we calling `unwrap()` on the lock? Rust Mutexes include a protection called "poisoning". If a thread panics while a Mutex is locked, that mutex is "poisoned" - and unwrapping it will fail. This is a safety measure against data corruption. If the thread crashed, can you trust the data anymore?

Also, we'll talk about *interior mutability* later that can make this clearer. Notice that once again, the Mutex is immutable - but we're locking it and changing the content.