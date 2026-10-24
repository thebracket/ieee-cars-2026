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
