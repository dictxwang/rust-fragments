use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;

// cargo run --bin about_arc_mutex
fn main() {

    // Arc配合Mutex使用，这里因为Mutex加锁，实际上线程方法没有真正的并行
    let map: HashMap<i32, i32> = HashMap::new();
    let arc: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(map));

    let mut handlers = vec![];
    for i in 1..10 {
        let arc_clone = Arc::clone(&arc);
        let handler = thread::spawn(move || {
            let mut m = arc_clone.lock().unwrap();
            m.insert(i, i);

            drop(m);  // 可以通过drop提前释放锁
            thread::sleep(time::Duration::from_secs(1));
        });
        handlers.push(handler);
    }

    for handler in handlers {
        // handler.join().unwrap();
        handler.join().expect("TODO: panic message");
    }
    for (k, v) in arc.lock().unwrap().iter() {
        println!("map item: key={}, value={}", k, v);
    }

}