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
