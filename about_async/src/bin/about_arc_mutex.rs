use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use rand::Rng;
use rand::{self, Rand};

fn use_hashmap() {

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

fn use_vec() {

    let vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
    
    let mut handlers = vec![];
    for i in 0..5 {
        let vec_clone = vec.clone();
        handlers.push(thread::spawn(move || {
            let mut rnd = rand::thread_rng();
            thread::sleep(time::Duration::from_secs(rnd.gen_range(1, 4)));
            let mut locked_vec = vec_clone.lock().unwrap();
            if locked_vec.is_empty() {
                locked_vec.push(i);
            } else {
                locked_vec[0] = i;
            }
            println!("vec 0 rewrite by {i}")
        }))
    }

    for handler in handlers {
        handler.join().expect("something wrong");
    }

    let final_value = vec.lock().unwrap()[0];
    let length = vec.lock().unwrap().len();
    println!("vec value at index 0 is {final_value}");
    println!("vec length is {length}");
}

// cargo run --bin about_arc_mutex
fn main() {

    // use_hashmap();
    use_vec();
}