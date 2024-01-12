use std::{thread, time};
use anyhow::{Result, Ok};
use chrono::Local;


fn spawn_blocking_many() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    for i in 1..10 {
        rt.spawn_blocking(move || {
            thread::sleep(time::Duration::from_secs(1));
            println!("spawn_blocking_many index:{}", i);
        });
    }
    // 上面for循环中的10个任务会同时阻塞1秒

    thread::sleep(time::Duration::from_secs(2));

    // sleep2秒后，继续执行last task
    rt.spawn_blocking(|| {
        println!("last task");
    });

    rt.shutdown_background();
}

fn block_on() {

    let rt_before = tokio::runtime::Runtime::new().unwrap();
    rt_before.spawn(async {

        for index in 0..10 {
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
            println!("frontrun is {:?}", index);
        }
    });

    // println!("before block_on");
    // let rt_middle = tokio::runtime::Runtime::new().unwrap();
    // rt_middle.block_on(async {
    //     tokio::time::sleep(std::time::Duration::from_secs(12)).await;
    // });
    // println!("after block_on");

    let rt_after = tokio::runtime::Runtime::new().unwrap();
    rt_after.spawn(async {

        for index in 0..10 {
            println!("backrun is {:?}", index);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    // println!("before sleep");
    thread::sleep(time::Duration::from_secs(15));
}

fn cost_detect() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    for _ in 0..100 {
        let st = Local::now().timestamp_micros();
        rt.spawn(async move {
            let now = Local::now().timestamp_micros();
            let cost = now - st;
            println!("Spawn cost {:?} micros", cost);

            let nn = Local::now().timestamp_micros();
            let cost = nn - now;
            println!("Cost {:?} micros", cost);
        });
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// cargo run --bin about_tokio_runtime
#[tokio::main]
async fn main() -> Result<()> {

    // spawn_blocking_many();
    // block_on();
    // cost_detect();

    println!("all functions finished.");
    loop {
        
    }
    Ok(())
}