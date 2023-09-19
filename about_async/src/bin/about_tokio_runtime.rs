use std::{thread, time};
use anyhow::{Result, Ok};

fn spawn() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async {
        thread::sleep(time::Duration::from_secs(3));
        // tokio::time::sleep(time::Duration::from_secs(5)).await;
        println!("spawn 01");
    });
    
    rt.spawn(async {
        thread::sleep(time::Duration::from_secs(3));
        // tokio::time::sleep(time::Duration::from_secs(5)).await;
        println!("spawn 02");
    });

    thread::sleep(time::Duration::from_secs(6));
    rt.shutdown_background();
}

fn spawn_blocking() {

    let rt1 = tokio::runtime::Runtime::new().unwrap();
    let rt2 = tokio::runtime::Runtime::new().unwrap();
    // spawn_blocking会阻塞同一个runtime的线程
    rt1.spawn_blocking(|| {
        thread::sleep(time::Duration::from_secs(5));
        println!("[3] rt1: spawn_blocking");
    });

    // 会被rt1.spawn_blocking阻塞
    rt1.spawn(async{
        thread::sleep(time::Duration::from_secs(3));
        println!("[2] rt1: spawn");
    });

    // 不会被rt1.spawn_blocking阻塞
    rt2.spawn(async {
        thread::sleep(time::Duration::from_secs(1));
        println!("[1] rt2: spawn");
    });
    thread::sleep(time::Duration::from_secs(6));

    rt1.shutdown_background();
    rt2.shutdown_background();
}

async fn sleep_println(duration: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;
    println!("sleep and println");
}

fn spawn_outer() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn({
        sleep_println(3)
    });
    
    rt.spawn({
        sleep_println(4)
    });

    thread::sleep(time::Duration::from_secs(5));
    rt.shutdown_background();
}

fn spawn_many() {

    // 默认worker_threads数等于cpu核数
    let rt = tokio::runtime::Runtime::new().unwrap();
    for i in 1..10 {
        rt.spawn(async move {
            thread::sleep(time::Duration::from_secs(1));
            println!("spawn_many index:{}", i);
        });
    }

    // 通过sleep确保上面的async task都执行完成，另一种方法是spawn_many_another中采取的方法
    thread::sleep(time::Duration::from_secs(5));
    rt.shutdown_background();
}

async fn spawn_many_another() {

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().worker_threads(8).build().unwrap();
    let mut handlers = vec![];
    for i in 1..20 {
        let handler = rt.spawn(async move {
            thread::sleep(time::Duration::from_secs(1));
            println!("spawn_many_another index:{}", i);
            12345678
        });
        handlers.push(handler);
    }

    // 通过handler.await确保所有的async task执行完成，而后执行shutdown
    // 从而避免出现runtime内部不能shutdown另一个runtime的panic
    for handler in handlers {
        let val = handler.await.expect("something wrong");
        println!("spawn_many_another return val:{val}");
    }
    rt.shutdown_background();
}

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

// cargo run --bin about_tokio_runtime
#[tokio::main]
async fn main() -> Result<()> {
    // spawn();
    // spawn_blocking();
    // spawn_outer();
    // spawn_many();
    spawn_many_another().await;
    // spawn_blocking_many();

    println!("all functions finished.");
    Ok(())
}