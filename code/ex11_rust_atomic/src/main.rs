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
