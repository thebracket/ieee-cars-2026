fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect(); // Shorthand for building a vector [0,1,2 .. 4999]
    let mut thread_handles = Vec::new();
    // `chunks` takes the size of each chunk, not the number of chunks. Round up
    // so we get at most `N_THREADS` chunks. `max(1)` keeps `chunks` happy if
    // `to_add` is empty.
    let chunk_size = to_add.len().div_ceil(N_THREADS).max(1);
    let chunks = to_add.chunks(chunk_size);

    // Notice that each chunk is a *slice* - a reference - to part of the array.
    for chunk in chunks {
        // So we *move* the chunk into its own vector, taking ownership and
        // passing that ownership to the thread. This adds a `memcpy` call
        // to your code, but avoids ownership issues.
        let my_chunk = chunk.to_owned();

        // Each thread sums its own chunk. You could use .sum() for this!
        thread_handles.push(std::thread::spawn(move || {
            let mut sum = 0;
            for i in my_chunk {
                sum += i;
            }
            sum
        }));
    }

    // Sum the sums from each thread.
    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }
    println!("Sum is {sum}");
}
