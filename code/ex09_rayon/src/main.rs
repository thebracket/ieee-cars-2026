use std::time::Instant;
use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).into_par_iter().all(|i| n % i != 0 )
}

fn main() {
    // Print primes below 1,000
    let now = Instant::now();
    let numbers: Vec<u64> = (2 .. 10_000).collect();
    let elapsed = now.elapsed();
    let mut primes: Vec<&u64> = numbers.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    primes.sort();
    println!("{primes:?}");
    println!("It took {} us to find {} primes", elapsed.as_micros(), primes.len());
}
