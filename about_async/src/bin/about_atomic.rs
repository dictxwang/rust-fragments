use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time;
use std::sync::Arc;
use rand::Rng;

fn main() {
    let count = Arc::new(AtomicI32::new(10));

    for _ in 1..10 {
        let count = Arc::clone(&count);
        thread::spawn(move|| {
            let mut reg = rand::thread_rng();
            thread::sleep(time::Duration::from_secs(reg.gen_range(1, 3)));
            println!("{:?}", count.fetch_or(0, Ordering::SeqCst));
        });
    }

    thread::spawn(move|| {
        thread::sleep(time::Duration::from_secs(2));
        count.store(20, Ordering::SeqCst);
    });

    thread::sleep(time::Duration::from_secs(5));
}