use std::{collections::HashMap, sync::{Mutex, Arc}};

use anyhow::Result;
use chrono::Local;

fn map() {
    let mut map = HashMap::new();
    for i in 0..10000 {
        map.insert(i, i);
    }

    for i in 0..10000 {

        let ss = Local::now().timestamp_micros();
        map.get(&i).unwrap();
        let cost = Local::now().timestamp_micros() - ss;
        println!("{:?}", cost);
    }
}

async fn put(map: &Arc<Mutex<HashMap<i32, i32>>>, value: i32) {

    map.lock().unwrap().insert(1, value);
}

async fn mutex_map() {

    let map = Mutex::new(HashMap::new());
    let arc = Arc::new(map);

    for i in 0..10000 {
        put(&arc, i).await;
    }

    let arc_read = arc.clone();

    tokio::spawn(async move {
        for i in 0..10000 {
            let ss = Local::now().timestamp_micros();
            put(&arc, i).await;
            let cost = Local::now().timestamp_micros() - ss;
            println!("put cost {:?}", cost);
        }
    });

    for i in 0..10000 {

        let ss = Local::now().timestamp_micros();
        let _ = arc_read.lock().unwrap().get(&1).unwrap().clone();
        let cost = Local::now().timestamp_micros() - ss;
        println!("get cost {:?}", cost);
    }
}

fn time() {

    for _ in 0..1000 {
        let start = Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        Local::now().timestamp_micros();
        // std::thread::sleep(std::time::Duration::from_millis(2));
        let cost = Local::now().timestamp_micros() - start;
        println!("{}", cost);
    }
}

// cargo run --bin about_high_performance
#[tokio::main]
pub async fn main() -> Result<()> {

    // mutex_map().await;
    time();

    loop {
        
    }
    Ok(())
}