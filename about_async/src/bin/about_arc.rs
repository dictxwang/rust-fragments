use std::thread;
use std::time;
use std::sync::Arc;
use rand;
use rand::Rng;

fn main() {
    // Arc引用的内容不能改变
    let apple = Arc::new(10);

    for _ in 1..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            let mut reg = rand::thread_rng();
            thread::sleep(time::Duration::from_secs(reg.gen_range(1, 3)));
            println!("{:?}", apple);
        });
    }
    thread::sleep(time::Duration::from_secs(5));
}