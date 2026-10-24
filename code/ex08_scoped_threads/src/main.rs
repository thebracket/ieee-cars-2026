use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    // `chunks` takes the size of each chunk, not the number of chunks.
    let chunk_size = to_add.len().div_ceil(N_THREADS).max(1);
    let chunks = to_add.chunks(chunk_size);
    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || {
                let mut sum = 0;
                for i in chunk {
                    sum += i;
                }
                sum
            });
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<u32>()
    });
    println!("Sum is {sum}");
}
