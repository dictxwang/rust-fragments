use std::sync::Mutex;

fn first() {

    let mv:Mutex<Vec<i32>> = Mutex::new(vec![]);

    let mut locked_vec = mv.lock().unwrap();
    locked_vec[0] = 21;
    println!("item at index 0 is {}", locked_vec[0]);
}

// cargo run --bin about_mutex_vec
fn main() {

    first();
}