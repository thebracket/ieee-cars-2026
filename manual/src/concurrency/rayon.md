# Making it Easy with Rayon

> The code for this is in `code/ex09_rayon`.


A library named "Rayon" is the gold-standard for easy thread-based concurrency in Rust. It actually uses another crate (`crossbeam`) under the hood, but it provides a much simpler interface for the most common use cases. Rayon can help you with a lot of tasks. Let's work through using it.

## Parallel Iterators

Let's start by adding Rayon to the project:

```bash
cargo add rayon
```

Probably the nicest addition Rayon bring is `par_iter`. The majority of things you can do with an iterator, you can auto-parallelize with `par_iter`. For example:

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (0 .. 1_000_000).collect();
    let sum = numbers.par_iter().sum::<u64>();
    println!("{sum}");
}
```

Rayon creates a thread-pool (1 thread per CPU), with a job queue. The queue implements work-stealing (no idle threads), and supports "sub-tasks" - a task can wait for another task to complete. It really is as simple as using `par_iter()` (for an iterator of references), `par_iter_mut()` (for an iterator of mutable references), or `into_par_iter()` (for an iterator of values that moves the values).

Let's do another test, this time with nested tasks. We'll use a really inefficient function for finding prime numbers:

```rust
use std::time::Instant;
use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).into_par_iter().all(|i| n % i != 0 )
}

fn main() {
    // Print all primes below 10,000
    let numbers: Vec<u64> = (2 .. 10_000).collect();
    // Time only the search and sort, not the setup or printing
    let now = Instant::now();
    let mut primes: Vec<&u64> = numbers.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    primes.sort();
    let elapsed = now.elapsed();
    println!("{primes:?}");
    println!("It took {} us to find {} primes", elapsed.as_micros(), primes.len());
}
```

The stopwatch starts after the input vector is built and stops after the primes are
sorted, so the microsecond count measures the parallel prime search plus the sort —
not vector creation and not printing the result.
