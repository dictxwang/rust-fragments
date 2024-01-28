use std::{collections::HashMap, sync::{Mutex, Arc}};

use std::thread;
use std::time;
use once_cell::sync::Lazy;
use rand::Rng;
use anyhow::Result;
use chrono::Local;
use tokio::runtime;

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

pub fn process(index: i64) {

    // let mut reg = rand::thread_rng();
    thread::sleep(std::time::Duration::from_micros(20));
    // println!("process: {}", index);

}

pub async fn async_process(index: i64) {

    // let mut reg = rand::thread_rng();
    tokio::time::sleep(std::time::Duration::from_micros(20)).await;
    // println!("async process: {}", index);

}

static RT_EXECUTOR: Lazy<runtime::Runtime> = Lazy::new(|| 
    runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap()
);

pub async fn speed() {

    let rt = runtime::Runtime::new().unwrap();

    rt.spawn(async {
        let mut index = 0i64;
        loop {
            process(index);
            // RT_EXECUTOR.spawn(async_process(index));
            println!("process: {}", index);
            index += 1;
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
} 

// cargo run --bin about_high_performance
#[tokio::main]
pub async fn main() -> Result<()> {

    // mutex_map().await;
    // time();
    speed().await;

    loop {
        
    }
}